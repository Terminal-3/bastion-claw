#!/usr/bin/env bash
set -euo pipefail

# Run the deterministic Rust-side Reborn E2E gate.
# Usage:
#   scripts/reborn-e2e-rust.sh              # all groups
#   scripts/reborn-e2e-rust.sh architecture # boundary + host runtime spine
#   scripts/reborn-e2e-rust.sh runtimes     # dispatcher/runtime/process lanes
#   scripts/reborn-e2e-rust.sh substrates   # event/network/secret substrates
#
# Extra cargo test args can be passed through CARGO_TEST_ARGS, for example:
#   CARGO_TEST_ARGS='-- --nocapture' scripts/reborn-e2e-rust.sh architecture

group="${1:-all}"
extra_args=${CARGO_TEST_ARGS:-"-- --nocapture"}

run_test() {
  local package="$1"
  local test_name="$2"
  echo "::group::cargo test -p ${package} --test ${test_name}"
  # shellcheck disable=SC2086 # extra_args intentionally expands into cargo's trailing args.
  cargo test -p "${package}" --test "${test_name}" ${extra_args}
  echo "::endgroup::"
}

run_architecture() {
  run_test t3claw_architecture reborn_dependency_boundaries
  run_test t3claw_host_runtime host_runtime_contract
  run_test t3claw_host_runtime host_runtime_services_contract
  run_test t3claw_host_runtime reborn_e2e_gate
  run_test t3claw_host_runtime reborn_invoke_vertical_slice
  run_test t3claw_host_runtime runtime_http_egress_contract
  run_test t3claw_host_runtime builtin_obligation_handler_contract
  run_test t3claw_host_runtime obligation_services_composition_contract
  run_test t3claw_host_runtime production_trust_contract
  run_test t3claw_capabilities capability_boundary_contract
  run_test t3claw_capabilities capability_host_contract
  run_test t3claw_capabilities capability_host_dispatcher_integration
  run_test t3claw_capabilities capability_host_process_integration
  run_test t3claw_capabilities capability_host_run_state_contract
  run_test t3claw_capabilities capability_host_spawn_contract
  run_test t3claw_capabilities capability_obligation_handler_contract
}

run_runtimes() {
  run_test t3claw_dispatcher boundary_contract
  run_test t3claw_dispatcher dispatch_contract
  run_test t3claw_dispatcher event_dispatch_contract
  run_test t3claw_dispatcher runtime_dispatcher_integration
  run_test t3claw_dispatcher vertical_slice_contract
  run_test t3claw_wasm wasm_dispatch_integration
  run_test t3claw_wasm wasm_http_adapter_contract
  run_test t3claw_wasm wit_tool_runtime_contract
  run_test t3claw_scripts script_dispatch_integration
  run_test t3claw_scripts script_http_adapter_contract
  run_test t3claw_scripts script_runner_contract
  run_test t3claw_mcp mcp_adapter_contract
  run_test t3claw_mcp mcp_dispatch_integration
  run_test t3claw_processes process_dispatch_integration
  run_test t3claw_processes process_host_contract
  run_test t3claw_processes process_services_contract
  run_test t3claw_processes process_store_contract
}

run_substrates() {
  run_test t3claw_events durable_log_contract
  run_test t3claw_filesystem catalog_contract
  run_test t3claw_filesystem filesystem_contract
  run_test t3claw_network boundary_contract
  run_test t3claw_network network_http_egress_contract
  run_test t3claw_network network_policy_contract
  run_test t3claw_secrets boundary_contract
  run_test t3claw_secrets secret_store_contract
  run_test t3claw_resources resource_governor_contract
  run_test t3claw_run_state approval_resolution_contract
  run_test t3claw_run_state run_state_contract
  run_test t3claw_approvals approval_resolution_contract
  run_test t3claw_approvals boundary_contract
  run_test t3claw_authorization boundary_contract
  run_test t3claw_authorization capability_access_contract
  run_test t3claw_authorization capability_lease_contract
}

case "${group}" in
  architecture)
    run_architecture
    ;;
  runtimes)
    run_runtimes
    ;;
  substrates)
    run_substrates
    ;;
  all)
    run_architecture
    run_runtimes
    run_substrates
    ;;
  *)
    echo "unknown Reborn E2E group: ${group}" >&2
    echo "expected one of: architecture, runtimes, substrates, all" >&2
    exit 2
    ;;
esac
