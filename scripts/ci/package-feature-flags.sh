#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 1 ]; then
  echo "usage: $0 <package>" >&2
  exit 2
fi

case "$1" in
  t3claw_reborn_cli)
    printf '%s\n' "--features webui-v2-beta,slack-v2-host-beta"
    ;;
  t3claw_product_adapters)
    printf '%s\n' "--features test-support,host-auth-mint"
    ;;
  t3claw_product_workflow)
    printf '%s\n' "--features test-support"
    ;;
  t3claw_product_workflow_storage)
    printf '%s\n' "--features libsql"
    ;;
  t3claw_reborn_composition)
    printf '%s\n' "--features test-support,webui-v2-beta,slack-v2-host-beta,libsql"
    ;;
  t3claw_reborn)
    printf '%s\n' "--features root-llm-provider,libsql-secrets,libsql-restart-tests,webui-user-store"
    ;;
  t3claw_reborn_event_store)
    printf '%s\n' "--features libsql"
    ;;
  t3claw_reborn_webui_ingress)
    printf '%s\n' "--features dev-in-memory-session"
    ;;
  t3claw_webui_v2 | t3claw_webui_v2_static)
    printf '%s\n' "--features webui-v2-beta"
    ;;
  *)
    ;;
esac
