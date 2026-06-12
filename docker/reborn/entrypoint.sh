#!/bin/sh
set -eu

is_truthy() {
  case "${1:-}" in
    1|true|TRUE|yes|YES) return 0 ;;
    *) return 1 ;;
  esac
}

railway_runtime_detected() {
  [ -n "${RAILWAY_ENVIRONMENT:-}" ] \
    || [ -n "${RAILWAY_PROJECT_ID:-}" ] \
    || [ -n "${RAILWAY_SERVICE_ID:-}" ]
}

railway_volume_mount=""
if [ -n "${RAILWAY_VOLUME_MOUNT_PATH:-}" ]; then
  railway_volume_mount="${RAILWAY_VOLUME_MOUNT_PATH%/}"
  if [ -z "$railway_volume_mount" ]; then
    railway_volume_mount="/"
  fi
fi

if [ -n "${T3CLAW_REBORN_HOME:-}" ]; then
  T3CLAW_REBORN_HOME="${T3CLAW_REBORN_HOME%/}"
elif [ -n "$railway_volume_mount" ]; then
  case "$railway_volume_mount" in
    */t3claw-reborn) T3CLAW_REBORN_HOME="$railway_volume_mount" ;;
    *) T3CLAW_REBORN_HOME="$railway_volume_mount/t3claw-reborn" ;;
  esac
else
  T3CLAW_REBORN_HOME="/data/t3claw-reborn"
fi
export T3CLAW_REBORN_HOME
if [ -n "${T3CLAW_REBORN_DEFAULT_CONFIG:-}" ]; then
  default_config="$T3CLAW_REBORN_DEFAULT_CONFIG"
elif [ "${T3CLAW_REBORN_PROFILE:-}" = "production" ] || [ "${T3CLAW_REBORN_PROFILE:-}" = "migration-dry-run" ]; then
  default_config="/opt/t3claw/reborn/config.production.toml"
else
  default_config="/opt/t3claw/reborn/config.toml"
fi
config_path="$T3CLAW_REBORN_HOME/config.toml"

case "$default_config" in
  /opt/t3claw/*) ;;
  *)
    echo "T3CLAW_REBORN_DEFAULT_CONFIG must be under /opt/t3claw: $default_config" >&2
    exit 1
    ;;
esac

case "$default_config" in
  *"/../"*|*"/.."|*"../"*|*"/."|*"/./"*)
    echo "T3CLAW_REBORN_DEFAULT_CONFIG must not contain relative path segments: $default_config" >&2
    exit 1
    ;;
esac

if [ ! -f "$config_path" ]; then
  mkdir -p "$T3CLAW_REBORN_HOME"
  tmp_config="${config_path}.tmp.$$"
  trap 'rm -f "$tmp_config"' EXIT HUP INT TERM
  cp "$default_config" "$tmp_config"
  if ! ln "$tmp_config" "$config_path" 2>/dev/null && [ ! -f "$config_path" ]; then
    echo "failed to install default Reborn config at $config_path" >&2
    exit 1
  fi
  rm -f "$tmp_config"
  trap - EXIT HUP INT TERM
fi

effective_profile="${T3CLAW_REBORN_PROFILE:-}"
if [ -z "$effective_profile" ]; then
  effective_profile="$(sed -n 's/^[[:space:]]*profile[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "$config_path" | sed -n '1p')"
fi
if [ -z "$effective_profile" ]; then
  effective_profile="local-dev"
fi

case "$effective_profile" in
  production|migration-dry-run)
    if ! grep -q '^[[:space:]]*\[storage\][[:space:]]*$' "$config_path" \
      || ! grep -q '^[[:space:]]*\[policy\][[:space:]]*$' "$config_path"
    then
      echo "T3CLAW_REBORN_PROFILE=$effective_profile requires $config_path to contain [storage] and [policy]." >&2
      echo "The existing config looks like a stale local-dev seed; remove it to let the entrypoint install $default_config, or migrate it manually." >&2
      exit 1
    fi
    ;;
esac

if railway_runtime_detected \
  && ! is_truthy "${T3CLAW_REBORN_ALLOW_EPHEMERAL_RAILWAY:-}"
then
  case "$effective_profile" in
    local-dev|local-dev-yolo)
      if [ -z "$railway_volume_mount" ]; then
        echo "Railway deployment using profile=$effective_profile requires a persistent volume for T3CLAW_REBORN_HOME=$T3CLAW_REBORN_HOME." >&2
        echo "Attach a Railway volume mounted at /data (or set T3CLAW_REBORN_HOME under RAILWAY_VOLUME_MOUNT_PATH)." >&2
        echo "Set T3CLAW_REBORN_ALLOW_EPHEMERAL_RAILWAY=true only for disposable test deployments." >&2
        exit 1
      fi
      case "$T3CLAW_REBORN_HOME" in
        "$railway_volume_mount"|"$railway_volume_mount"/*) ;;
        *)
          echo "Railway deployment using profile=$effective_profile requires T3CLAW_REBORN_HOME=$T3CLAW_REBORN_HOME to be under RAILWAY_VOLUME_MOUNT_PATH=$railway_volume_mount." >&2
          echo "Unset T3CLAW_REBORN_HOME to use $railway_volume_mount/t3claw-reborn, or set T3CLAW_REBORN_ALLOW_EPHEMERAL_RAILWAY=true only for disposable tests." >&2
          exit 1
          ;;
      esac
      ;;
  esac
fi

host="${T3CLAW_REBORN_SERVE_HOST:-127.0.0.1}"
port="${PORT:-${T3CLAW_REBORN_SERVE_PORT:-3000}}"

resolve_env_placeholder_arg() {
  case "$1" in
    '$T3CLAW_REBORN_SERVE_HOST'|'${T3CLAW_REBORN_SERVE_HOST}')
      printf '%s\n' "$host"
      ;;
    '$PORT'|'${PORT}'|'$T3CLAW_REBORN_SERVE_PORT'|'${T3CLAW_REBORN_SERVE_PORT}')
      printf '%s\n' "$port"
      ;;
    *)
      printf '%s\n' "$1"
      ;;
  esac
}

if [ "$#" -gt 0 ]; then
  original_arg_count="$#"
  while [ "$original_arg_count" -gt 0 ]; do
    arg="$(resolve_env_placeholder_arg "$1")"
    shift
    original_arg_count=$((original_arg_count - 1))
    set -- "$@" "$arg"
  done
  exec t3claw-reborn "$@"
fi

set -- serve --host "$host" --port "$port"

if is_truthy "${T3CLAW_REBORN_CONFIRM_HOST_ACCESS:-}"; then
  set -- "$@" --confirm-host-access
fi

exec t3claw-reborn "$@"
