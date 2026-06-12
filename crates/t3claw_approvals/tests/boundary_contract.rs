#[test]
fn approvals_crate_stays_out_of_runtime_and_host_workflow_crates() {
    let manifest_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let manifest = std::fs::read_to_string(&manifest_path)
        .unwrap_or_else(|error| panic!("failed to read {manifest_path:?}: {error}"));
    let dependencies = dependencies_section(&manifest);

    for forbidden in [
        "t3claw_capabilities",
        "t3claw_dispatcher",
        "t3claw_processes",
        "t3claw_host_runtime",
        "t3claw_resources",
        "t3claw_extensions",
        "t3claw_wasm",
        "t3claw_scripts",
        "t3claw_mcp",
    ] {
        assert!(
            !dependencies.contains(forbidden),
            "t3claw_approvals should resolve approval records into leases/audit without depending on {forbidden}"
        );
    }
}

fn dependencies_section(manifest: &str) -> &str {
    manifest
        .split_once("[dependencies]")
        .and_then(|(_, rest)| rest.split_once("[dev-dependencies]").map(|(deps, _)| deps))
        .expect("Cargo.toml must contain [dependencies] before [dev-dependencies]")
}
