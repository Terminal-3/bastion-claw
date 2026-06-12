#[test]
fn dispatcher_crate_does_not_depend_on_higher_level_workflow_crates() {
    let manifest_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let manifest = std::fs::read_to_string(&manifest_path)
        .unwrap_or_else(|error| panic!("failed to read {manifest_path:?}: {error}"));

    for forbidden in [
        "t3claw_authorization",
        "t3claw_capabilities",
        "t3claw_wasm",
        "t3claw_scripts",
        "t3claw_mcp",
    ] {
        assert!(
            !manifest.contains(forbidden),
            "t3claw_dispatcher examples/tests should exercise the dispatcher boundary directly, not depend on higher-level workflow crate {forbidden}"
        );
    }
}
