//! Canonical, typed model of the `t3n_delegation_token` secret.
//!
//! The delegation-token secret is consumed by two paths that historically
//! parsed it independently and drifted apart:
//!
//!   - the **write path** ([`channels::web::handlers::secrets`]) validates the
//!     full secret shape on PUT and rejects a malformed value with a structured
//!     HTTP 400 (`{ code, field, reason }`);
//!   - the **read path** ([`tools::mcp::client`]) parses the same secret to
//!     select a credential and inject it into a `t3n-mcp` tool call.
//!
//! This module is the single source of truth for the secret's structure so the
//! two paths can never disagree about what a valid token looks like.
//!
//! ## Two strictness levels, on purpose
//!
//! The two callers do not validate to the same depth, and that asymmetry is
//! deliberate, not accidental drift:
//!
//!   - [`DelegationToken::parse`] is the **strict** entry point used at write
//!     time. It runs the complete shape check (required string fields,
//!     `user_sig` → 65 bytes, `agent_pubkey` → 33 bytes, `credential_jcs` →
//!     inner JSON with the required claim fields, `org_did` regex). An operator
//!     learns about a malformed token at upload time rather than ~30 s later
//!     when `runPayroll` fails.
//!   - [`DelegationToken::parse_lenient`] is the **structural** entry point used
//!     at read time. It performs the outer-JSON parse and the
//!     single-vs-role-map branch, but defers per-credential validation: the read
//!     path only needs the `credential_jcs` / `user_sig` strings, and a token
//!     that already passed write-time validation is trusted thereafter. Routing
//!     the read path through the strict check would re-reject tokens it has
//!     always accepted.
//!
//! Both share the same outer-JSON parsing and the same single-vs-role-map
//! branching — the logic that actually drifted — so the shapes can no longer
//! diverge.

use std::collections::BTreeMap;

use base64::Engine as _;
use regex::Regex;
use std::sync::LazyLock;

/// Expected length, in bytes, of an Ethereum signature (`r ‖ s ‖ v`).
///
/// Mirrored from `client/t3n-sdk/src/client/delegation.ts`.
const ETH_SIG_LEN: usize = 65;

/// Expected length, in bytes, of a compressed secp256k1 public key.
///
/// Mirrored from `client/t3n-sdk/src/client/delegation.ts`.
const AGENT_PUBKEY_LEN: usize = 33;

/// A fully-qualified Trinity organisation DID: `did:t3n:<40 lowercase hex>`.
static ORG_DID_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^did:t3n:[0-9a-f]{40}$").expect("static org-DID regex compiles") // safety: compile-time literal pattern; a malformed regex is a programmer error caught on first use, not a runtime input failure
});

/// One delegation credential — the `{ credential_jcs, user_sig, agent_pubkey }`
/// triple stored either at the top level (legacy single token) or under each
/// role of the multi-role map.
///
/// The three fields are kept as their raw on-the-wire strings. Structural
/// validation is performed on demand by [`CredentialEntry::validate`] so the
/// lenient read path can hold an entry it never byte-checks while the strict
/// write path verifies every entry it parses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CredentialEntry {
    pub credential_jcs: String,
    pub user_sig: String,
    pub agent_pubkey: String,
}

impl CredentialEntry {
    /// Read the three required string fields out of a JSON object, without any
    /// further validation. Used by both parse paths to build the entry.
    fn from_json(value: &serde_json::Value) -> Result<Self, DelegationTokenError> {
        let get_str = |field: &'static str| -> Result<String, DelegationTokenError> {
            match value.get(field) {
                None => Err(DelegationTokenError::MissingField { field }),
                Some(v) => v
                    .as_str()
                    .map(str::to_string)
                    .ok_or(DelegationTokenError::WrongType {
                        field,
                        expected: "string",
                    }),
            }
        };

        Ok(Self {
            credential_jcs: get_str("credential_jcs")?,
            user_sig: get_str("user_sig")?,
            agent_pubkey: get_str("agent_pubkey")?,
        })
    }

    /// Full structural validation of the credential (write-path strictness):
    ///
    ///   - `user_sig` is base64url and decodes to exactly 65 bytes;
    ///   - `agent_pubkey` is hex (optional `0x` prefix) or base64url and decodes
    ///     to exactly 33 bytes;
    ///   - `credential_jcs` is base64url, decodes to JSON, and that inner JSON
    ///     carries `org_did` / `vc_id` / `user_did` / `not_before_secs` /
    ///     `not_after_secs`;
    ///   - the inner `org_did` matches `did:t3n:<40 lowercase hex>`.
    fn validate(&self) -> Result<(), DelegationTokenError> {
        let b64u = base64::engine::general_purpose::URL_SAFE_NO_PAD;

        // user_sig: base64url → 65 bytes.
        let sig_bytes = b64u
            .decode(self.user_sig.trim_end_matches('='))
            .map_err(|e| DelegationTokenError::InvalidB64u {
                field: "user_sig",
                reason: e.to_string(),
            })?;
        if sig_bytes.len() != ETH_SIG_LEN {
            return Err(DelegationTokenError::WrongByteLength {
                field: "user_sig",
                expected: ETH_SIG_LEN,
                actual: sig_bytes.len(),
            });
        }

        // agent_pubkey: hex (optional 0x prefix) or base64url → 33 bytes.
        let pubkey_bytes = {
            let s = self.agent_pubkey.trim_start_matches("0x");
            // Attempt hex first: must be exactly 66 hex chars (33 bytes × 2).
            if s.len() == 66 && s.chars().all(|c| c.is_ascii_hexdigit()) {
                hex::decode(s).map_err(|e| DelegationTokenError::InvalidB64u {
                    field: "agent_pubkey",
                    reason: e.to_string(),
                })?
            } else {
                b64u.decode(self.agent_pubkey.trim_end_matches('='))
                    .map_err(|e| DelegationTokenError::InvalidB64u {
                        field: "agent_pubkey",
                        reason: e.to_string(),
                    })?
            }
        };
        if pubkey_bytes.len() != AGENT_PUBKEY_LEN {
            return Err(DelegationTokenError::WrongByteLength {
                field: "agent_pubkey",
                expected: AGENT_PUBKEY_LEN,
                actual: pubkey_bytes.len(),
            });
        }

        // credential_jcs: base64url → inner JSON.
        let jcs_bytes = b64u
            .decode(self.credential_jcs.trim_end_matches('='))
            .map_err(|e| DelegationTokenError::InvalidB64u {
                field: "credential_jcs",
                reason: e.to_string(),
            })?;
        let inner: serde_json::Value = serde_json::from_slice(&jcs_bytes).map_err(|e| {
            DelegationTokenError::InnerJsonInvalid {
                reason: e.to_string(),
            }
        })?;

        // Required inner string claim fields.
        let get_inner_str = |field: &'static str| -> Result<&str, DelegationTokenError> {
            match inner.get(field) {
                None => Err(DelegationTokenError::MissingInnerField { field }),
                Some(v) => v.as_str().ok_or(DelegationTokenError::WrongInnerType {
                    field,
                    expected: "string",
                }),
            }
        };

        let org_did = get_inner_str("org_did")?.to_string();
        get_inner_str("vc_id")?;
        get_inner_str("user_did")?;

        // `not_before_secs` / `not_after_secs` are numbers on the wire but are
        // accepted as either number or string per the spec note; only presence
        // is checked here.
        if inner.get("not_before_secs").is_none() {
            return Err(DelegationTokenError::MissingInnerField {
                field: "not_before_secs",
            });
        }
        if inner.get("not_after_secs").is_none() {
            return Err(DelegationTokenError::MissingInnerField {
                field: "not_after_secs",
            });
        }

        // org_did must be a fully-qualified did:t3n:<40 lowercase hex>.
        if !ORG_DID_RE.is_match(&org_did) {
            return Err(DelegationTokenError::InvalidOrgDidShape { value: org_did });
        }

        Ok(())
    }
}

/// The two supported shapes of the `t3n_delegation_token` secret.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DelegationToken {
    /// Legacy single, role-agnostic credential — a top-level
    /// `{ credential_jcs, user_sig, agent_pubkey }` with no `roles` key.
    Single(CredentialEntry),

    /// MVP2 multi-role map — `{ "roles": { "<role>": {credential}, … },
    /// "default_role": "<role>" }`. Each role names its own credential;
    /// `default_role` names the role used when a call supplies no `as_role`.
    ///
    /// `default_role` is `None` when the key is absent or not a string. The two
    /// callers treat that differently: the write path requires a present,
    /// string-typed `default_role` that names a role (enforced in
    /// [`DelegationToken::parse`]); the read path defers entirely to
    /// [`DelegationToken::select`], which only consults it when no `as_role` was
    /// supplied. Capturing it leniently here keeps the read path from rejecting a
    /// token it historically tolerated.
    RoleMap {
        roles: BTreeMap<String, CredentialEntry>,
        default_role: Option<String>,
    },
}

impl DelegationToken {
    /// Parse and **fully validate** the secret (write-path strictness).
    ///
    /// Runs the outer-JSON parse, branches on the presence of a `roles` key,
    /// validates each credential entry, and — for the role-map shape — requires
    /// `default_role` to name a present role. This is the function the PUT
    /// handler uses to reject a malformed token at upload time.
    pub(crate) fn parse(value: &str) -> Result<Self, DelegationTokenError> {
        let token = Self::parse_structure(value)?;
        match &token {
            Self::Single(entry) => entry.validate()?,
            Self::RoleMap { roles, .. } => {
                for entry in roles.values() {
                    entry.validate()?;
                }
                // The write path requires `default_role` to be present, a
                // string, and to name a role in the map. `parse_structure`
                // captures it leniently (as `None` when absent or non-string),
                // so the strict presence/type check is re-derived from the raw
                // value here to preserve the historical 400 field/reason.
                let raw: serde_json::Value =
                    serde_json::from_str(value).map_err(|e| DelegationTokenError::InvalidJson {
                        reason: e.to_string(),
                    })?;
                let default_role = match raw.get("default_role") {
                    None => {
                        return Err(DelegationTokenError::MissingField {
                            field: "default_role",
                        });
                    }
                    Some(v) => v.as_str().ok_or(DelegationTokenError::WrongType {
                        field: "default_role",
                        expected: "string",
                    })?,
                };
                if !roles.contains_key(default_role) {
                    return Err(DelegationTokenError::DefaultRoleUnknown {
                        role: default_role.to_string(),
                    });
                }
            }
        }
        Ok(token)
    }

    /// Parse the secret **structurally only** (read-path strictness).
    ///
    /// Performs the outer-JSON parse and the single-vs-role-map branch, reading
    /// the credential string fields, but does not byte-check the credentials or
    /// require `default_role` to be present. The read path only needs the
    /// `credential_jcs` / `user_sig` strings, and a token that already passed
    /// write-time validation is trusted thereafter; `default_role` is consulted
    /// (and its absence reported) only at [`DelegationToken::select`] time, and
    /// only when no `as_role` was supplied.
    ///
    /// **Invariant:** the injection path (`parse_lenient` + [`DelegationToken::select`])
    /// does not re-byte-validate the stored credential triple. This is sound
    /// because the secret PUT handler's strict [`DelegationToken::parse`] is the
    /// only sanctioned writer of the `t3n_delegation_token` secret, so every
    /// stored token is fully validated at write time. The read path must not be
    /// the entry point through which an unvalidated triple reaches a dispatch — if
    /// a second writer is ever added, that writer must run the strict `parse`, or
    /// this invariant breaks and the read path would need its own byte-check.
    pub(crate) fn parse_lenient(value: &str) -> Result<Self, DelegationTokenError> {
        Self::parse_structure(value)
    }

    /// Shared outer-JSON parse and single-vs-role-map branch.
    ///
    /// For the role-map shape, `default_role` is captured if present but is not
    /// required here — the two callers differ on when its absence is an error
    /// (see [`DelegationToken::parse`] vs [`DelegationToken::select`]).
    fn parse_structure(value: &str) -> Result<Self, DelegationTokenError> {
        let token: serde_json::Value =
            serde_json::from_str(value).map_err(|e| DelegationTokenError::InvalidJson {
                reason: e.to_string(),
            })?;

        if let Some(roles) = token.get("roles") {
            let roles_obj = roles.as_object().ok_or(DelegationTokenError::WrongType {
                field: "roles",
                expected: "object",
            })?;

            let mut roles = BTreeMap::new();
            for (role, cred) in roles_obj {
                roles.insert(role.clone(), CredentialEntry::from_json(cred)?);
            }

            // `default_role` is captured leniently: absent or non-string both
            // become `None`. A role-map token with `as_role` supplied never
            // consults it (read path), so rejecting a non-string value here
            // would break tokens the read path historically tolerated. The write
            // path re-derives the strict presence/type check in `parse`.
            let default_role = token
                .get("default_role")
                .and_then(|v| v.as_str())
                .map(str::to_string);

            return Ok(Self::RoleMap {
                roles,
                default_role,
            });
        }

        // Legacy single-credential shape.
        Ok(Self::Single(CredentialEntry::from_json(&token)?))
    }

    /// Select the credential entry for a call, by approver role.
    ///
    /// - **Single** — role-agnostic. A supplied `as_role` has nothing to select
    ///   against, so it is rejected rather than silently ignored (which would
    ///   mask a misconfiguration). With no `as_role`, the sole credential is
    ///   returned.
    /// - **RoleMap** — `as_role` picks the entry; absent that, `default_role` is
    ///   used. A role (requested or default) that is missing from the map is a
    ///   hard error — never a silent fallback — so a wrong-role call surfaces
    ///   rather than dispatching the wrong approver's credential. A role-map
    ///   token with neither an `as_role` nor a usable `default_role` cannot pick
    ///   a credential and errors.
    pub(crate) fn select(
        &self,
        as_role: Option<&str>,
    ) -> Result<&CredentialEntry, DelegationTokenError> {
        match self {
            Self::Single(entry) => {
                if let Some(role) = as_role {
                    return Err(DelegationTokenError::AsRoleAgainstSingle {
                        role: role.to_string(),
                    });
                }
                Ok(entry)
            }
            Self::RoleMap {
                roles,
                default_role,
            } => {
                let role = match as_role {
                    Some(r) => r.to_string(),
                    None => default_role
                        .clone()
                        .ok_or(DelegationTokenError::MissingDefaultRoleForSelect)?,
                };

                roles
                    .get(&role)
                    .ok_or_else(|| DelegationTokenError::UnknownRole {
                        role,
                        available: roles.keys().cloned().collect(),
                    })
            }
        }
    }
}

/// Every way the `t3n_delegation_token` secret can fail to parse, validate, or
/// be selected from.
///
/// One enum serves both callers:
///   - the write handler turns it into the existing structured 400 body via
///     [`DelegationTokenError::field`] + [`DelegationTokenError::reason`]; the
///     variant set and the strings produced are unchanged from the previous
///     hand-rolled validator, so the HTTP contract is byte-identical.
///   - the read path surfaces the human-readable message via `Display`, which
///     reproduces the strings the injection path emitted before this module
///     existed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DelegationTokenError {
    // ── Write-path shape errors (drive the structured 400 body) ──────────────
    InvalidJson {
        reason: String,
    },
    MissingField {
        field: &'static str,
    },
    WrongType {
        field: &'static str,
        expected: &'static str,
    },
    InvalidB64u {
        field: &'static str,
        reason: String,
    },
    WrongByteLength {
        field: &'static str,
        expected: usize,
        actual: usize,
    },
    InnerJsonInvalid {
        reason: String,
    },
    MissingInnerField {
        field: &'static str,
    },
    WrongInnerType {
        field: &'static str,
        expected: &'static str,
    },
    InvalidOrgDidShape {
        value: String,
    },
    DefaultRoleUnknown {
        role: String,
    },

    // ── Read-path selection errors (surfaced via Display) ────────────────────
    /// An `as_role` was supplied against a legacy single-credential token.
    AsRoleAgainstSingle {
        role: String,
    },
    /// A role-map token had no `as_role` and no usable `default_role`.
    MissingDefaultRoleForSelect,
    /// The requested (or default) role is absent from the role map.
    UnknownRole {
        role: String,
        available: Vec<String>,
    },
}

impl DelegationTokenError {
    /// The `field` value of the structured 400 body, for the write path.
    ///
    /// Only the write-path shape variants are reachable here — selection
    /// variants never originate from [`DelegationToken::parse`]. They map to
    /// `<root>` defensively rather than panicking, since they carry no field.
    pub(crate) fn field(&self) -> &str {
        match self {
            Self::InvalidJson { .. } => "<root>",
            Self::MissingField { field } => field,
            Self::WrongType { field, .. } => field,
            Self::InvalidB64u { field, .. } => field,
            Self::WrongByteLength { field, .. } => field,
            Self::InnerJsonInvalid { .. } => "credential_jcs",
            Self::MissingInnerField { field } => field,
            Self::WrongInnerType { field, .. } => field,
            Self::InvalidOrgDidShape { .. } => "org_did",
            Self::DefaultRoleUnknown { .. } => "default_role",
            Self::AsRoleAgainstSingle { .. }
            | Self::MissingDefaultRoleForSelect
            | Self::UnknownRole { .. } => "<root>",
        }
    }

    /// The `reason` value of the structured 400 body, for the write path.
    ///
    /// The strings for the write-path shape variants are byte-for-byte identical
    /// to those the previous hand-rolled validator produced; the secrets.rs
    /// tests assert against them.
    pub(crate) fn reason(&self) -> String {
        match self {
            Self::InvalidJson { reason } => reason.clone(),
            Self::MissingField { field } => format!("required field '{field}' is missing"),
            Self::WrongType { field, expected } => {
                format!("field '{field}' must be a {expected}")
            }
            Self::InvalidB64u { field, reason } => {
                format!("field '{field}' is not valid base64url: {reason}")
            }
            Self::WrongByteLength {
                field,
                expected,
                actual,
            } => {
                format!(
                    "field '{field}' must be {expected} bytes after base64url decode, got {actual}"
                )
            }
            Self::InnerJsonInvalid { reason } => {
                format!("credential_jcs does not decode to valid JSON: {reason}")
            }
            Self::MissingInnerField { field } => {
                format!("credential_jcs is missing required inner field '{field}'")
            }
            Self::WrongInnerType { field, expected } => {
                format!("credential_jcs inner field '{field}' must be a {expected}")
            }
            Self::InvalidOrgDidShape { value } => {
                format!(
                    "org_did '{value}' must match did:t3n:<40 lowercase hex> \
                     (e.g. did:t3n:a1b2c3…)"
                )
            }
            Self::DefaultRoleUnknown { role } => {
                format!("default_role '{role}' is not present in roles")
            }
            // Selection variants never reach the 400-body path, but Display is
            // the authoritative wording for them; reuse it here.
            Self::AsRoleAgainstSingle { .. }
            | Self::MissingDefaultRoleForSelect
            | Self::UnknownRole { .. } => self.to_string(),
        }
    }
}

impl std::fmt::Display for DelegationTokenError {
    /// Human-readable message for the read path.
    ///
    /// The selection-variant strings reproduce, verbatim, the messages the
    /// injection path emitted before this module existed (prefixed `t3n-mcp:`).
    /// The shape variants reuse the write-path wording so a single token has one
    /// description regardless of which caller rejected it.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // ── Selection variants ──────────────────────────────────────────
            Self::AsRoleAgainstSingle { role } => write!(
                f,
                "t3n-mcp: call supplied as_role='{role}' but the stored delegation token is a \
                 single legacy credential with no role map — re-run the SDK bootstrap to upload \
                 the per-role credentials, or omit `as_role`."
            ),
            Self::MissingDefaultRoleForSelect => write!(
                f,
                "t3n-mcp: delegation token has a 'roles' map but no 'default_role', \
                 and the call supplied no 'as_role' — cannot pick a credential"
            ),
            Self::UnknownRole { role, available } => {
                let mut available: Vec<&str> = available.iter().map(String::as_str).collect();
                available.sort_unstable();
                write!(
                    f,
                    "t3n-mcp: no delegation credential for role '{role}' — \
                     the stored token only provides roles [{}]. Pass `as_role` matching one of \
                     those, or re-run the SDK bootstrap to upload the missing role.",
                    available.join(", ")
                )
            }

            // ── Structural variants the read path can hit at parse time ──────
            Self::InvalidJson { reason } => {
                write!(
                    f,
                    "t3n-mcp: stored delegation token is not valid JSON: {reason}"
                )
            }
            Self::WrongType { field: "roles", .. } => {
                write!(f, "t3n-mcp: delegation token 'roles' must be a JSON object")
            }
            Self::MissingField { field } => write!(
                f,
                "t3n-mcp: delegation token is missing required field '{field}'"
            ),

            // Remaining shape variants (only reachable from the strict write
            // path, never surfaced to the read path): reuse the structured-body
            // wording so any future read-path use still prints coherently.
            _ => write!(f, "{}: {}", self.field(), self.reason()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b64u(bytes: &[u8]) -> String {
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
    }

    fn valid_credential_jcs_b64u(org_did: &str) -> String {
        let inner = serde_json::json!({
            "vc_id": "AAAAAAAAAAAAAAAAAAAAAA",
            "user_did": "did:t3n:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "org_did": org_did,
            "not_before_secs": 1700000000u64,
            "not_after_secs": 1800000000u64,
        });
        b64u(inner.to_string().as_bytes())
    }

    const VALID_ORG_DID: &str = "did:t3n:a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2";

    /// A fully-valid single credential (passes strict `validate`).
    fn valid_single() -> serde_json::Value {
        serde_json::json!({
            "credential_jcs": valid_credential_jcs_b64u(VALID_ORG_DID),
            "user_sig": b64u(&[0xABu8; ETH_SIG_LEN]),
            "agent_pubkey": b64u(&[0xCDu8; AGENT_PUBKEY_LEN]),
        })
    }

    /// A structurally-present but byte-invalid credential (loose read-path
    /// fixture): the three fields are present strings, but `user_sig` is not a
    /// 65-byte signature.
    fn loose_single(sig: &str) -> serde_json::Value {
        serde_json::json!({
            "credential_jcs": valid_credential_jcs_b64u(VALID_ORG_DID),
            "user_sig": sig,
            "agent_pubkey": "pubkey-value",
        })
    }

    // ── parse (strict / write path) ─────────────────────────────────────────

    #[test]
    fn parse_single_valid() {
        let token = DelegationToken::parse(&valid_single().to_string()).expect("valid single");
        assert!(matches!(token, DelegationToken::Single(_)));
    }

    #[test]
    fn parse_role_map_valid() {
        let cred = valid_single();
        let json = serde_json::json!({
            "roles": { "cfo": cred.clone(), "hr_admin": cred },
            "default_role": "cfo",
        });
        let token = DelegationToken::parse(&json.to_string()).expect("valid role map");
        match token {
            DelegationToken::RoleMap {
                roles,
                default_role,
            } => {
                assert_eq!(default_role.as_deref(), Some("cfo"));
                assert_eq!(roles.len(), 2);
            }
            other => panic!("expected RoleMap, got {other:?}"),
        }
    }

    #[test]
    fn parse_role_map_unknown_default_role() {
        let json = serde_json::json!({
            "roles": { "cfo": valid_single() },
            "default_role": "ceo",
        });
        let err = DelegationToken::parse(&json.to_string()).unwrap_err();
        assert!(matches!(
            err,
            DelegationTokenError::DefaultRoleUnknown { .. }
        ));
        assert_eq!(err.field(), "default_role");
    }

    #[test]
    fn parse_role_map_missing_default_role() {
        let json = serde_json::json!({ "roles": { "cfo": valid_single() } });
        let err = DelegationToken::parse(&json.to_string()).unwrap_err();
        assert!(matches!(
            err,
            DelegationTokenError::MissingField {
                field: "default_role"
            }
        ));
    }

    #[test]
    fn parse_rejects_bad_inner_credential() {
        // hr_admin has a 39-byte user_sig → per-entry validate fails.
        let bad = serde_json::json!({
            "credential_jcs": valid_credential_jcs_b64u(VALID_ORG_DID),
            "user_sig": b64u(&[0xABu8; 39]),
            "agent_pubkey": b64u(&[0xCDu8; AGENT_PUBKEY_LEN]),
        });
        let json = serde_json::json!({
            "roles": { "cfo": valid_single(), "hr_admin": bad },
            "default_role": "cfo",
        });
        let err = DelegationToken::parse(&json.to_string()).unwrap_err();
        assert!(matches!(
            err,
            DelegationTokenError::WrongByteLength {
                field: "user_sig",
                ..
            }
        ));
    }

    #[test]
    fn parse_rejects_outer_json() {
        let err = DelegationToken::parse("not-json{{{").unwrap_err();
        assert!(matches!(err, DelegationTokenError::InvalidJson { .. }));
        assert_eq!(err.field(), "<root>");
    }

    // ── parse_lenient (read path) ───────────────────────────────────────────

    #[test]
    fn parse_lenient_accepts_byte_invalid_single() {
        // The read path must accept a token whose user_sig is not a real 65-byte
        // signature — it never byte-checks. Strict parse would reject this.
        let json = loose_single("usig-value").to_string();
        assert!(DelegationToken::parse(&json).is_err());
        let token = DelegationToken::parse_lenient(&json).expect("lenient accepts loose single");
        assert!(matches!(token, DelegationToken::Single(_)));
    }

    #[test]
    fn parse_lenient_role_map_without_default_role() {
        // A role map with no default_role parses leniently; the absence is only
        // an error at select time when no as_role is supplied.
        let json = serde_json::json!({
            "roles": { "cfo": loose_single("usig-cfo") },
        });
        let token = DelegationToken::parse_lenient(&json.to_string()).expect("lenient role map");
        assert!(matches!(token, DelegationToken::RoleMap { .. }));
    }

    // ── select ──────────────────────────────────────────────────────────────

    #[test]
    fn select_single_no_role() {
        let token =
            DelegationToken::parse_lenient(&loose_single("usig-value").to_string()).unwrap();
        let entry = token.select(None).expect("single selects with no role");
        assert_eq!(entry.user_sig, "usig-value");
    }

    #[test]
    fn select_single_rejects_as_role() {
        let token =
            DelegationToken::parse_lenient(&loose_single("usig-value").to_string()).unwrap();
        let err = token.select(Some("cfo")).unwrap_err();
        assert!(matches!(
            err,
            DelegationTokenError::AsRoleAgainstSingle { .. }
        ));
        assert!(err.to_string().contains("as_role") && err.to_string().contains("legacy"));
    }

    #[test]
    fn select_role_map_by_as_role() {
        let json = serde_json::json!({
            "roles": {
                "cfo": loose_single("usig-cfo"),
                "hr_admin": loose_single("usig-hr"),
            },
            "default_role": "cfo",
        });
        let token = DelegationToken::parse_lenient(&json.to_string()).unwrap();
        let entry = token.select(Some("hr_admin")).expect("selects hr_admin");
        assert_eq!(entry.user_sig, "usig-hr");
    }

    #[test]
    fn select_role_map_default() {
        let json = serde_json::json!({
            "roles": {
                "cfo": loose_single("usig-cfo"),
                "hr_admin": loose_single("usig-hr"),
            },
            "default_role": "hr_admin",
        });
        let token = DelegationToken::parse_lenient(&json.to_string()).unwrap();
        let entry = token.select(None).expect("falls back to default_role");
        assert_eq!(entry.user_sig, "usig-hr");
    }

    #[test]
    fn select_role_map_unknown_role() {
        let json = serde_json::json!({
            "roles": {
                "cfo": loose_single("usig-cfo"),
                "hr_admin": loose_single("usig-hr"),
                "junior": loose_single("usig-junior"),
            },
            "default_role": "cfo",
        });
        let token = DelegationToken::parse_lenient(&json.to_string()).unwrap();
        let err = token.select(Some("treasurer")).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("treasurer"), "names the requested role: {msg}");
        assert!(
            msg.contains("cfo") && msg.contains("hr_admin") && msg.contains("junior"),
            "lists available roles sorted: {msg}"
        );
    }

    #[test]
    fn select_role_map_no_default_and_no_as_role_errors() {
        let json = serde_json::json!({
            "roles": { "cfo": loose_single("usig-cfo") },
        });
        let token = DelegationToken::parse_lenient(&json.to_string()).unwrap();
        let err = token.select(None).unwrap_err();
        assert!(matches!(
            err,
            DelegationTokenError::MissingDefaultRoleForSelect
        ));
        assert!(err.to_string().contains("cannot pick a credential"));
    }
}
