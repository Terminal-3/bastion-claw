//! Cross-backend adversarial parity suite for the reborn hook
//! [`PredicateStateBackend`] (durable-backend PR 4/4).
//!
//! This crate ships **no production code**. It exists solely to host the
//! integration-test binaries under `tests/` that drive all three concrete
//! `PredicateStateBackend` implementations —
//!
//! - [`t3claw_hooks::predicate_state::InMemoryPredicateStateBackend`] (the
//!   in-process backend; PR 1/4),
//! - [`t3claw_hooks_postgres::PostgresPredicateStateBackend`] (PR 2/4),
//! - [`t3claw_hooks_libsql::LibSqlPredicateStateBackend`] (PR 3/4),
//!
//! through the **same scripted input sequence** and cross-assert that their
//! observable outputs are byte-for-byte identical: same counts, same sums,
//! same dedup behaviour, same fail-closed [`WindowOverflow`] at the per-key
//! cap, same per-tenant LRU eviction victims.
//!
//! [`WindowOverflow`]: t3claw_hooks::predicate_state::PredicateBackendError::WindowOverflow
//!
//! See `tests/parity_matrix.rs` for the load-bearing matrix and the
//! durable-backend doc `crates/t3claw_hooks/docs/successors/03-persistent-counter.md`
//! for the guarantees this suite proves.
