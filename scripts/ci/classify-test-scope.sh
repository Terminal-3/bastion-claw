#!/usr/bin/env bash
set -euo pipefail

has_core_code=false
docs_only=true
has_legacy_tests=false
has_reborn_tests=false

is_docs_only_path() {
  local path="$1"
  case "$path" in
    docs/*|.github/ISSUE_TEMPLATE/*|.github/pull_request_template.md)
      return 0
      ;;
    *.md)
      case "$path" in
        */*) return 1 ;;
        *) return 0 ;;
      esac
      ;;
    *)
      return 1
      ;;
  esac
}

is_shared_test_path() {
  local path="$1"
  case "$path" in
    Cargo.toml|Cargo.lock|build.rs|providers.json|Dockerfile)
      return 0
      ;;
    scripts/ci/classify-test-scope.sh|scripts/ci/test-classify-test-scope.sh|scripts/ci/package-feature-flags.sh)
      return 0
      ;;
    .github/workflows/test.yml|.github/workflows/reborn-tests.yml|.github/workflows/reborn-integration.yml|.github/workflows/reborn-e2e.yml|.github/workflows/nightly-deep-ci.yml)
      return 0
      ;;
    crates/t3claw_common/*|crates/t3claw_host_api/*|crates/t3claw_host_runtime/*|crates/t3claw_loop_support/*)
      return 0
      ;;
    crates/t3claw_filesystem/*|crates/t3claw_memory/*|crates/t3claw_events/*|crates/t3claw_event_projections/*|crates/t3claw_event_streams/*)
      return 0
      ;;
    crates/t3claw_capabilities/*|crates/t3claw_secrets/*|crates/t3claw_network/*|crates/t3claw_runtime_policy/*)
      return 0
      ;;
    crates/t3claw_authorization/*|crates/t3claw_run_state/*|crates/t3claw_approvals/*|crates/t3claw_resources/*)
      return 0
      ;;
    crates/t3claw_auth/*|crates/t3claw_trust/*|crates/t3claw_turns/*|crates/t3claw_agent_loop/*|crates/t3claw_threads/*)
      return 0
      ;;
    crates/t3claw_prompt_envelope/*|crates/t3claw_hooks/*|crates/t3claw_first_party_extensions/*|crates/t3claw_llm/*)
      return 0
      ;;
    crates/t3claw_embeddings/*|crates/t3claw_safety/*|crates/t3claw_skills/*|crates/t3claw_oauth/*)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

is_reborn_test_path() {
  local path="$1"
  case "$path" in
    docs/reborn/*|scripts/reborn-e2e-rust.sh|scripts/ci/run-reborn-root-partition.sh|tests/reborn_*|tests/support/reborn/*|tests/e2e/scenarios/test_reborn_*)
      return 0
      ;;
    crates/t3claw_architecture/*)
      return 0
      ;;
    crates/t3claw_reborn/*|crates/t3claw_reborn_*/*)
      return 0
      ;;
    crates/t3claw_product_*/*|crates/t3claw_slack_v2_adapter/*|crates/t3claw_telegram_v2_adapter/*)
      return 0
      ;;
    crates/t3claw_wasm_product_adapters/*|crates/t3claw_webui_v2/*|crates/t3claw_webui_v2_static/*)
      return 0
      ;;
    crates/t3claw_conversations/*|crates/t3claw_outbound/*|crates/t3claw_triggers/*)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

is_code_path() {
  local path="$1"
  case "$path" in
    src/*|crates/*|channels-src/*|tools-src/*|tests/*|migrations/*)
      return 0
      ;;
    Cargo.toml|Cargo.lock|Dockerfile|build.rs|providers.json)
      return 0
      ;;
    scripts/check_no_panics.py|scripts/check_gateway_boundaries.py|scripts/build-wasm-extensions.sh|scripts/check-version-bumps.sh|scripts/reborn-e2e-rust.sh|scripts/ci/*)
      return 0
      ;;
    .github/workflows/*.yml|.github/actions/install-cargo-component/*|.github/dependabot.yml|.github/labeler.yml)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

while IFS= read -r path || [ -n "$path" ]; do
  [ -n "$path" ] || continue

  if ! is_docs_only_path "$path"; then
    docs_only=false
  fi

  if is_code_path "$path"; then
    has_core_code=true
  fi

  if is_shared_test_path "$path"; then
    has_legacy_tests=true
    has_reborn_tests=true
  elif is_reborn_test_path "$path"; then
    has_reborn_tests=true
  elif is_code_path "$path"; then
    has_legacy_tests=true
  fi
done

cat <<EOF
docs_only=${docs_only}
has_core_code=${has_core_code}
has_legacy_tests=${has_legacy_tests}
has_reborn_tests=${has_reborn_tests}
EOF
