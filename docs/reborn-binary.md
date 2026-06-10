# `t3claw-reborn` standalone binary

`t3claw-reborn` is the standalone executable boundary for Reborn. It is separate from the current `t3claw` binary so Reborn boot, config, state, and runtime composition can evolve without accidentally invoking v1 runtime paths.

This binary is available as the workspace package `t3claw_reborn_cli` and builds the executable named `t3claw-reborn`.

## Current status

`t3claw-reborn` is an early operator/testing surface, not the default T3Claw runtime.

It currently supports:

```bash
t3claw-reborn --help
t3claw-reborn channels list
t3claw-reborn channels list --json
t3claw-reborn channels list --verbose
t3claw-reborn completion --shell bash
t3claw-reborn completion --shell zsh
t3claw-reborn config path
t3claw-reborn doctor
t3claw-reborn extension search github
t3claw-reborn extension search github --json
t3claw-reborn extension install github-mcp
t3claw-reborn extension activate github-mcp
t3claw-reborn extension remove github-mcp
t3claw-reborn hooks list
t3claw-reborn hooks list --json
t3claw-reborn hooks list --verbose
t3claw-reborn logs
t3claw-reborn logs --json
t3claw-reborn logs --verbose
t3claw-reborn models list
t3claw-reborn models list --json
t3claw-reborn models status
t3claw-reborn models status --json
t3claw-reborn profile list
t3claw-reborn profile list --json
t3claw-reborn repl
t3claw-reborn run
t3claw-reborn run --confirm-host-access
t3claw-reborn serve
t3claw-reborn serve --confirm-host-access
t3claw-reborn skills list
t3claw-reborn skills list --json
t3claw-reborn skills list --verbose
```

It intentionally does not yet support:

- replacing `t3claw` behavior;
- daemon/service installation;
- web gateway/UI startup;
- v1 config, DB, settings, or secrets migration;
- production extension/tool execution;
- long-lived Reborn runtime services.

## Commands

### `channels list`

Reports configured Reborn channels without resolving Reborn home, reading v1 channel config, or creating directories.

The Reborn channel registry is not wired yet, so the command currently reports an explicit empty surface:

```bash
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- channels list
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- channels list --json
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- channels list --verbose
```

Expected fields include:

- `configured: 0`
- `status: not-wired`
- `v1_state: not-used`

### `extension`

Searches and manages local-dev Reborn extensions through the same lifecycle facade exposed to product surfaces. Available extension packages are read from `/system/extensions`, which maps to `<reborn-home>/local-dev/system/extensions` for the local-dev profile.

```bash
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- extension search github
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- extension search github --json
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- extension install github-mcp
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- extension activate github-mcp
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- extension remove github-mcp
```

The commands are scoped to Reborn boot/config resolution and do not create or read v1 state directories.

Expected fields include:

- `phase`
- `package_ref.id` for package-specific commands
- `payload.kind`
- `payload.count` and `payload.extensions[].package_ref.id` for search
- `payload.installed`, `payload.activated`, or `payload.removed` for lifecycle mutations

### `completion`

Generates shell completion scripts without resolving Reborn home, reading v1 state, or creating directories.

```bash
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- completion --shell zsh > t3claw-reborn.zsh
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- completion --shell bash > t3claw-reborn.bash
```

The zsh output keeps the v1 CLI guard around `compdef` so the generated script is safe when zsh completion functions are not loaded yet.

### `config path`

Shows the resolved Reborn state root, its source, selected profile, and explicit v1-state status without creating directories.

```bash
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- config path
```

Expected fields include:

- `reborn_home`
- `home_source`
- `profile`
- `v1_state: not-used`

`config path`, `doctor`, and other read-only surfaces do not create Reborn
state or seed config files.

### `doctor`

Validates and reports Reborn boot configuration without creating state directories or starting runtime services.

```bash
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- doctor
```

Expected fields include:

- `reborn_home`
- `home_source`
- `profile`
- `v1_state: not-used`
- `driver_registry: initialized`

### `hooks list`

Reports configured Reborn hooks without resolving Reborn home, reading v1 hook config, or creating directories.

The Reborn hook registry is not wired yet, so the command currently reports an explicit empty surface:

```bash
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- hooks list
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- hooks list --json
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- hooks list --verbose
```

Expected fields include:

- `configured: 0`
- `status: not-wired`
- `v1_state: not-used`

### `logs`

Reports Reborn log availability without resolving Reborn home, reading v1 gateway logs, or creating directories.

The Reborn log source is not wired yet, so the command currently reports an explicit empty surface:

```bash
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- logs
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- logs --json
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- logs --verbose
```

Expected fields include:

- `entries: 0`
- `status: not-wired`
- `v1_state: not-used`

### `models list` / `models status`

Shows Reborn model purpose slots and route status without resolving Reborn home, reading v1 provider settings, or creating directories.

Routes are not configurable through Reborn CLI yet, so the command currently reports `not-configured` routes for built-in slots:

```bash
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- models list
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- models list --json
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- models status
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- models status --json
```

Expected fields include:

- `default`
- `mission`
- `routes: not-configured`
- `v1_state: not-used`

### `profile list`

Lists the supported Reborn boot profiles without resolving Reborn home, reading v1 state, or creating directories.

```bash
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- profile list
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- profile list --json
```

Supported profiles:

- `local-dev` (default)
- `local-dev-yolo`
- `production`
- `migration-dry-run`

Select a profile with `T3CLAW_REBORN_PROFILE=<profile>`.

### `run`

Starts the standalone Reborn runtime and reads messages from stdin. The no-profile path targets the planned AgentLoop runtime (`reborn-planned-default`). Without model provider environment variables, the runtime still starts but messages fail cleanly because no LLM gateway is wired.

```bash
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- run
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- run --message "hello"
```

Use `--dry-run` for the side-effect-free readiness snapshot:

```bash
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- run --dry-run
```

When `$T3CLAW_REBORN_HOME/config.toml` is missing, the first stateful
runtime start through `run`, `repl`, or feature-gated `serve` seeds a sparse
`config.toml` containing `api_version` and the safe `local-dev` boot profile.
It intentionally does not seed `[llm.default]`, so env-only model selection
continues to work. `run --dry-run`, diagnostics, and read-only commands remain
side-effect-free. One-off environment selections such as
`T3CLAW_REBORN_PROFILE=local-dev-yolo` are not persisted into the seeded
file.

Expected fields include:

- `binary: t3claw-reborn`
- `version`
- `reborn_home`
- `home_source`
- `profile`
- `v1_state: not-used`
- `runtime_driver: planned-agent-loop`
- `driver_registry: initialized`
- `local_runtime_shell_readiness: ready`
- `planned_default_profile: available`

For `T3CLAW_REBORN_PROFILE=local-dev-yolo`, `run`, `repl`, and `serve` require `--confirm-host-access` before the runtime receives trusted-laptop host access. Confirmed access mounts the host home through `/host`; Unix-style raw home aliases are also accepted when they can be represented as scoped mount aliases.

When `serve --confirm-host-access` grants trusted-laptop access, `serve` refuses non-loopback listeners such as `0.0.0.0`. Bind to `127.0.0.1` or `::1`, or use a less privileged profile for non-loopback test listeners.

For `T3CLAW_REBORN_PROFILE=production`, `run` requires production storage
and an explicit runtime policy:

```toml
[storage]
backend = "postgres"
url_env = "T3CLAW_REBORN_POSTGRES_URL"
secret_master_key_env = "T3CLAW_REBORN_SECRET_MASTER_KEY"
# Optional; defaults to 16. Keep below the PostgreSQL server's max_connections
# after reserving capacity for migrations and operator sessions.
pool_max_size = 16

[policy]
deployment_mode = "hosted_multi_tenant"
default_profile = "secure_default"
```

Set `T3CLAW_REBORN_POSTGRES_URL` in the process environment, and set
`T3CLAW_REBORN_SECRET_MASTER_KEY` to independent cryptographic key material.
Remote managed PostgreSQL URLs must use TLS, for example `sslmode=require`.
The first production launch slice supports runtime policies that do not require
a tenant-sandbox process binding.

### `skills list`

Reports configured Reborn local-dev skills from `<reborn-home>/local-dev/skills`
and `<reborn-home>/local-dev/system/skills` through the Reborn composition
skill listing function. It does not read v1 skill discovery paths, and a missing
local-dev storage root is reported as an empty skill list without creating
directories.

```bash
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- skills list
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- skills list --json
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- skills list --verbose
```

Expected fields include:

- `configured: <count>`
- `source: reborn-local-dev`
- per-skill `name`, `source`, and `description` in text output
- per-skill `name`, `version`, `description`, `source`, `keywords`, `tags`,
  and `requires_skills` in JSON output

`--verbose` adds the resolved `profile`, `reborn_home`, `local_dev_root`, and
`owner_id`; text output also includes per-skill `version`, `keywords`, `tags`,
and `requires_skills` when present. `skills list` currently supports
`local-dev` and `local-dev-yolo` profiles and rejects `production` /
`migration-dry-run` until those catalog backends are wired.

## State and config root

Reborn must not use the current v1 T3Claw state root by default.

Home resolution precedence:

1. `T3CLAW_REBORN_HOME`
2. `~/.t3claw/reborn`

The resolver rejects unsafe or misleading homes, including empty paths, relative paths, filesystem root, parent-directory components, and known v1 state-root aliases such as `$HOME/.t3claw` or `T3CLAW_BASE_DIR`.

## Profiles

Use `T3CLAW_REBORN_PROFILE` to select the boot profile.

Supported values:

- `local-dev` (default)
- `local-dev-yolo`
- `production`
- `migration-dry-run`

Example:

```bash
T3CLAW_REBORN_HOME="$PWD/.reborn-home" \
T3CLAW_REBORN_PROFILE=production \
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- doctor
```

## Local smoke checks

Run these before changing Reborn CLI behavior:

```bash
cargo fmt --all -- --check
cargo test -p t3claw_reborn_cli
cargo test -p t3claw_reborn_config
cargo test -p t3claw_reborn model_slots_are_exposed_in_cli_display_order
cargo test -p t3claw_architecture reborn
cargo clippy -p t3claw_reborn_cli --all-targets -- -D warnings
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- --help
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- channels list
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- completion --shell zsh >/tmp/t3claw-reborn.zsh
T3CLAW_REBORN_HOME="$(mktemp -d)/reborn-home" \
  cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- config path
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- hooks list
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- logs
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- models status
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- profile list
T3CLAW_REBORN_HOME="$(mktemp -d)/reborn-home" \
  cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- run
cargo run -q -p t3claw_reborn_cli --bin t3claw-reborn -- skills list
```

## Adding commands

Future commands should follow the crate-local agent contract in:

```text
crates/t3claw_reborn_cli/AGENTS.md
```

Short version:

1. add one command module under `crates/t3claw_reborn_cli/src/commands/`;
2. register it in `commands::Command`;
3. resolve and pass `RebornCliContext` from dispatch only when the command needs boot config;
4. keep pure commands independent from Reborn home resolution;
5. add a binary smoke test through `env!("CARGO_BIN_EXE_t3claw-reborn")`;
6. avoid v1 runtime imports and v1 state mutation unless explicitly scoped and guarded.

Do not port the current `src/cli/*` command tree wholesale. Port commands one at a time, starting with Reborn-owned or read-only surfaces.

## Release packaging decision

`t3claw-reborn` is **not yet included in cargo-dist release artifacts**.

Current `dist plan --output-format=json` with `crates/t3claw_reborn_cli` marked `dist = false` emits only the root `t3claw` package artifacts. Removing `dist = false` alone is not enough to ship `t3claw-reborn` in the existing `t3claw-v*` release workflow because that workflow is shaped around the root `t3claw` package tag. Enabling a standalone `t3claw_reborn_cli` release also requires cargo-dist WiX metadata/template work and an explicit tag/versioning decision.

Follow-up issue: #3483 tracks packaging `t3claw-reborn` in release artifacts.

Until #3483 is resolved, keep:

```toml
[package.metadata.dist]
dist = false
```

in `crates/t3claw_reborn_cli/Cargo.toml` so releases do not silently claim to ship an unverified Reborn binary package.
