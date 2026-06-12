use crate::oauth_provider_client::{ExchangeScopePolicy, HostOAuthProviderSpec};

pub(crate) fn google_provider_spec() -> HostOAuthProviderSpec {
    HostOAuthProviderSpec {
        provider_id: t3claw_auth::GOOGLE_PROVIDER_ID,
        capability_id: "t3claw_auth.google_oauth",
        token_endpoint: t3claw_auth::GOOGLE_TOKEN_ENDPOINT,
        secret_handle_prefix: "google",
        resource: None,
        exchange_scope_policy: ExchangeScopePolicy::RequireProviderScope,
    }
}
