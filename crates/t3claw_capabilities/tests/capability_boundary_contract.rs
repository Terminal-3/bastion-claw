use std::fs;
use std::path::PathBuf;

#[test]
fn capabilities_crate_does_not_depend_on_concrete_runtime_or_dispatcher_crates() {
    let manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let manifest = fs::read_to_string(manifest_path).unwrap();
    let production_dependencies = manifest
        .split("\n[dev-dependencies]")
        .next()
        .unwrap_or(&manifest);
    for forbidden in [
        "t3claw_dispatcher",
        "t3claw_host_runtime",
        "t3claw_mcp",
        "t3claw_scripts",
        "t3claw_wasm",
        "t3claw_secrets",
        "t3claw_network",
    ] {
        assert!(
            !production_dependencies.contains(forbidden),
            "t3claw_capabilities production code must use neutral ports and must not depend on {forbidden}"
        );
    }
}
