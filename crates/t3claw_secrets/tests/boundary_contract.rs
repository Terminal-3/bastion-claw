#[test]
fn secrets_crate_does_not_depend_on_workflow_runtime_or_observability_crates() {
    let manifest_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let manifest = std::fs::read_to_string(&manifest_path)
        .unwrap_or_else(|error| panic!("failed to read {manifest_path:?}: {error}"));

    for forbidden in [
        "t3claw_authorization",
        "t3claw_approvals",
        "t3claw_capabilities",
        "t3claw_dispatcher",
        "t3claw_events",
        "t3claw_extensions",
        "t3claw_host_runtime",
        "t3claw_mcp",
        "t3claw_processes",
        "t3claw_resources",
        "t3claw_run_state",
        "t3claw_scripts",
        "t3claw_wasm",
    ] {
        assert!(
            !manifest.contains(forbidden),
            "t3claw_secrets must stay a low-level scoped secret service, not depend on {forbidden}"
        );
    }
}
