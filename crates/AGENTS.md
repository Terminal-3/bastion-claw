# T3Claw Crates Map

Instructions for AI coding assistants entering `crates/` on `reborn-integration`.

This file is a routing map, not a full architecture spec. Pick the crate(s) that match the change, then read crate-local guidance before editing:

1. `crates/<crate>/AGENTS.md` when present.
2. `crates/<crate>/CLAUDE.md` if present.
3. `crates/<crate>/CONTRACT.md` or `README.md` if present.
4. Matching `docs/reborn/contracts/*.md` when behavior crosses crate boundaries.

Do **not** eagerly load every crate guide. Use this map to choose.

## Branch and Workspace

This map was refreshed from `reborn-integration` after inspecting the workspace crate manifests, source layout, tests, and crate-local docs. Most crates have a crate-local `AGENTS.md`; when one is missing, load `CLAUDE.md`, `Cargo.toml`, and `src/lib.rs` instead.

Run crate work from repo root unless crate-local docs say otherwise.

```bash
cargo test -p <crate_name>
cargo clippy -p <crate_name> --all-targets --all-features -- -D warnings
cargo test -p t3claw_architecture
scripts/check-boundaries.sh
scripts/reborn-e2e-rust.sh
```

Use targeted crate tests first. Add `t3claw_architecture` when dependency edges or layer ownership change. Run Reborn e2e when turns, runtime lanes, host services, authorization, approvals, networking, secrets, product workflow, or capability dispatch change.

## Guidance Files

- `AGENTS.md` — crate-local agent entrypoint; read first.
- `CLAUDE.md` — crate guardrails/spec; read before changing behavior.
- `CONTRACT.md` — public cross-crate contract; update with semantic changes.
- `README.md` — helper/user/operator details.
- `docs/reborn/contracts/*.md` — Reborn source-of-truth contracts.
- `crates/t3claw_architecture` — mechanical dependency-boundary enforcement.

Treat crate-local `AGENTS.md` as the first file to load when it exists. Current workspace crates without one include `t3claw_hooks`, `t3claw_prompt_envelope`, `t3claw_reborn_traces`, and `t3claw_wasm_limiter`.

## Dependency Mental Model

Keep lower layers neutral. Product and runtime composition flows downward through typed contracts, not concrete shortcuts.

```text
common / host_api / prompt_envelope
  -> filesystem / memory / events / event_projections / event_streams / extensions / trust / resources
  -> secrets / network / outbound / run_state / authorization / approvals / runtime_policy / hooks
  -> host_runtime / processes / dispatcher / runtime lanes (scripts, mcp, wasm, wasm_limiter)
  -> turns / threads / agent_loop / loop_support / capabilities
  -> reborn composition / product adapters / product workflow / product workflow storage / CLI
  -> engine / llm / gateway / webui_v2 / webui_ingress / tui / root product integration
```

Boundary rule: if you need an upstream crate in a low-level crate, stop and check `crates/t3claw_architecture` plus matching Reborn contract.

## Crate Map

### Foundation and substrate

| Crate | Load first | Owns / go here for | Avoid moving in |
| --- | --- | --- | --- |
| `t3claw_common` | `t3claw_common/AGENTS.md`, `Cargo.toml` | Low-dependency shared types/utilities: app events, identity, trust-boundary helpers, paths, platform/env/timezone, attachment helpers. | Runtime orchestration, persistence, clients, policy, product domain logic. |
| `t3claw_host_api` | `t3claw_host_api/AGENTS.md`, `t3claw_host_api/CLAUDE.md`, `docs/reborn/contracts/host-api.md` | Neutral authority vocabulary: IDs, scopes, paths, actions, decisions, resources, approvals, audit, HTTP, dispatch, runtime-policy, trust types. | Runtime execution, persistence, HTTP clients, product workflow, policy engines. |
| `t3claw_prompt_envelope` | `Cargo.toml`, `src/lib.rs` | Leaf prompt-envelope helper: wraps model-visible snippets with closed-vocabulary source/trust labels, size limits, and instruction-hijack rejection. | Runtime orchestration, model routing, policy decisions, or free-form source labels. |
| `t3claw_architecture` | `t3claw_architecture/AGENTS.md`, `t3claw_architecture/CLAUDE.md` | Workspace architecture tests, Reborn dependency boundaries, composition-boundary checks. | Production runtime code or production deps. |

### Files, memory, events, projections

| Crate | Load first | Owns / go here for | Avoid moving in |
| --- | --- | --- | --- |
| `t3claw_filesystem` | `t3claw_filesystem/AGENTS.md`, `t3claw_filesystem/CLAUDE.md`, `docs/reborn/contracts/filesystem.md` | Root/scoped/composite filesystem, catalog, virtual path authority, backend containment, mount routing. | Memory-domain grammar, network/secrets/dispatcher/product workflow. |
| `t3claw_memory` | `t3claw_memory/AGENTS.md`, `t3claw_memory/CLAUDE.md`, `docs/reborn/contracts/memory.md` | Memory docs, `/memory` paths, metadata/schema, chunking, embeddings, search, indexer hooks, memory filesystem adapter, backend contracts. | Generic mount/catalog logic or product workflow. |
| `t3claw_events` | `t3claw_events/AGENTS.md`, `t3claw_events/CLAUDE.md`, `docs/reborn/contracts/events.md` | Typed redacted event/audit substrate, event envelopes, sinks/log traits, durable adapters. | SSE/WebSocket/product transport or projection policy. |
| `t3claw_event_projections` | `t3claw_event_projections/AGENTS.md`, `t3claw_event_projections/CLAUDE.md`, `docs/reborn/contracts/events-projections.md` | Event projection model, cursor/visibility contracts, product-facing projection boundaries. | Canonical event storage or transport delivery. |
| `t3claw_event_streams` | `t3claw_event_streams/AGENTS.md`, `t3claw_event_streams/CLAUDE.md`, `docs/reborn/contracts/events-projections.md` | Transport-neutral projection stream manager: admission, bounded subscription buffers, live/replay update delivery, lag/rebase signals, redaction validation. | Axum/SSE/WebSocket framing, product workflow submission, durable event-store adapters, raw runtime payloads. |
| `t3claw_reborn_event_store` | `t3claw_reborn_event_store/AGENTS.md`, `docs/reborn/contracts/events.md` | Reborn-owned durable event/audit store backends and fixtures. | Product projections, transport fanout, workflow policy. |
| `t3claw_reborn_traces` | `Cargo.toml`, `src/lib.rs` | Trace Commons / TraceDAO client surface: contribution pipeline, trace client, redaction helpers, conversation-message compatibility, and trace preview re-exports. | Reborn CLI command behavior, LLM provider routing, unredacted trace submission. |

### Authority, policy, state

| Crate | Load first | Owns / go here for | Avoid moving in |
| --- | --- | --- | --- |
| `t3claw_trust` | `t3claw_trust/AGENTS.md`, `t3claw_trust/CLAUDE.md`, `t3claw_trust/CONTRACT.md` | Host-controlled trust classes, policy sources, requested-vs-effective trust, invalidation. | Authorization grants, runtime dispatch, product workflow. |
| `t3claw_authorization` | `t3claw_authorization/AGENTS.md`, `t3claw_authorization/CLAUDE.md` | Grant matching, leases, dispatch/spawn authorization decisions, DB-backed auth state. | Execution, approvals, run-state persistence, prompting. |
| `t3claw_approvals` | `t3claw_approvals/AGENTS.md`, `t3claw_approvals/CLAUDE.md` | Exact-invocation approval requests, leases, resume coordination, approval events. | Reusable broad approvals or dispatch before fingerprinted lease claim. |
| `t3claw_run_state` | `t3claw_run_state/AGENTS.md`, `t3claw_run_state/CLAUDE.md` | Durable invocation state and approval request records. | Authorization policy, approval resolution, dispatch, runtime execution, process lifecycle. |
| `t3claw_resources` | `t3claw_resources/AGENTS.md`, `t3claw_resources/CLAUDE.md` | Reservation, reconciliation, release, quota accounting. | Runtime dispatch, product workflow, hidden costed work without reservation. |
| `t3claw_auth` | `t3claw_auth/AGENTS.md`, `t3claw_auth/CLAUDE.md`, `docs/reborn/contracts/auth-product.md` | Product-facing Reborn auth-flow, secure interaction, credential account, provider exchange, continuation, cleanup contracts and fakes. | V1 route handlers/pending maps, durable secret storage, raw provider HTTP, runtime injection, extension lifecycle mutation. |
| `t3claw_runtime_policy` | `t3claw_runtime_policy/AGENTS.md`, `t3claw_runtime_policy/CLAUDE.md`, `docs/reborn/contracts/runtime-profiles.md` | Runtime profile resolver and runtime selection policy. | Runtime startup, action dispatch, product strategy outside selection. |
| `t3claw_outbound` | `t3claw_outbound/AGENTS.md`, `t3claw_outbound/CLAUDE.md` | Metadata-only outbound egress policy, notification opt-in, projection subscription cursors, delivery attempt/status metadata. | Transport sends, concrete Slack/Telegram/Web payload validation, transcript/projection mutation. |
| `t3claw_hooks` | `t3claw_hooks/CLAUDE.md`, `Cargo.toml`, `src/lib.rs` | Reborn loop hook framework: trust-tiered hook contracts, sealed decision sinks, predicates, ordering, dispatch, telemetry, and failure policy. | Authority grants, runtime-policy bypasses, ambient secrets/network/filesystem handles, extension installation. |

### Host services and runtime lanes

| Crate | Load first | Owns / go here for | Avoid moving in |
| --- | --- | --- | --- |
| `t3claw_secrets` | `t3claw_secrets/AGENTS.md`, `t3claw_secrets/CLAUDE.md` | Secret metadata, encrypted repositories, leases, one-shot consumption, legacy/db stores. | Raw secret exposure, provider HTTP, injection beyond mediated handoff. |
| `t3claw_network` | `t3claw_network/AGENTS.md`, `t3claw_network/CLAUDE.md`, `docs/reborn/contracts/network.md` | Network policy boundary, URL targets, resolver, hardened transport, host/provider HTTP egress. | Runtime-lane behavior above boundary or manual credential injection. |
| `t3claw_host_runtime` | `t3claw_host_runtime/AGENTS.md`, `t3claw_host_runtime/CLAUDE.md` | Host-side Reborn service composition: production services, obligations, HTTP egress, redaction, secrets/network/resource mediation. | Product workflow, runtime-specific request shapes, duplicate network/secret logic. |
| `t3claw_processes` | `t3claw_processes/AGENTS.md`, `t3claw_processes/CLAUDE.md` | Process lifecycle, cancellation, stores, status/output helpers, `ProcessHost`, wrappers. | Authorization, approval policy, runtime lane internals beyond adapter contracts. |
| `t3claw_dispatcher` | `t3claw_dispatcher/AGENTS.md`, `t3claw_dispatcher/CLAUDE.md` | Already-authorized runtime routing through `RuntimeAdapter`, redacted dispatch results, event dispatch contracts. | Authorization, approvals, run-state, concrete runtime deps, product workflow. |
| `t3claw_scripts` | `t3claw_scripts/AGENTS.md`, `t3claw_scripts/CLAUDE.md` | Script runtime lane over host-mediated filesystem/events/resources/dispatcher/HTTP, Docker/backend output parsing. | Manual credentials, direct provider HTTP, duplicated dispatcher/process/resource policy. |
| `t3claw_mcp` | `t3claw_mcp/AGENTS.md`, `t3claw_mcp/CLAUDE.md` | MCP runtime lane, execution request/result types, JSON-RPC exchange, client abstraction, HTTP adapter, resource accounting. | Direct outbound networking, ad-hoc credential injection, product workflow. |
| `t3claw_wasm` | `t3claw_wasm/AGENTS.md`, `t3claw_wasm/CLAUDE.md`, `docs/reborn/contracts/wasm.md`, `wit/tool.wit` | WASM runtime lane, component/WIT bindings, limiter, store, host adapters, runtime config. | Privileged host effects outside mediated APIs; copied secrets/network/resource logic. |
| `t3claw_wasm_limiter` | `Cargo.toml`, `src/lib.rs` | Shared `wasmtime::ResourceLimiter` for WASM tool and hook runtimes. | Product adapter workflow, policy decisions, or runtime-specific side effects beyond limiter accounting. |
| `t3claw_wasm_sandbox_core` | `t3claw_wasm_sandbox_core/AGENTS.md`, `t3claw_wasm_sandbox_core/CLAUDE.md` | Shared WASM sandbox core primitives used below product adapters/runtime. | Product adapter workflow or host product policy. |

### Turns, threads, loops, engine

| Crate | Load first | Owns / go here for | Avoid moving in |
| --- | --- | --- | --- |
| `t3claw_turns` | `t3claw_turns/AGENTS.md`, `t3claw_turns/CLAUDE.md` | Host-layer turn coordination: requests/responses, coordinator, runner, run profiles, loop exit, memory/context handoff, turn store. | Product adapter rendering, raw runtime lanes, UI behavior. |
| `t3claw_threads` | `t3claw_threads/AGENTS.md`, `t3claw_threads/CLAUDE.md` | Canonical session thread/transcript service contracts, identifiers, tool-result references, db/in-memory stores. | Product delivery policy or model/provider behavior. |
| `t3claw_conversations` | `t3claw_conversations/AGENTS.md`, `t3claw_conversations/CLAUDE.md` | Conversation binding, session thread contracts, inbound/state store, libSQL/Postgres conversation persistence. | Capability runtime internals or UI transport. |
| `t3claw_agent_loop` | `t3claw_agent_loop/AGENTS.md`, `t3claw_agent_loop/CLAUDE.md` | Agent-loop framework state, planner/executor, strategy/family contracts, test support. | Product adapters, transport, concrete provider auth. |
| `t3claw_loop_support` | `t3claw_loop_support/AGENTS.md`, `t3claw_loop_support/CLAUDE.md` | Loop host support services: capability/input ports, allow sets, input queue, identity/skill context, cancellation. | Owning core loop strategy or runtime lane execution. |
| `t3claw_capabilities` | `t3claw_capabilities/AGENTS.md`, `t3claw_capabilities/CLAUDE.md` | Caller-facing `CapabilityHost` invoke/resume/spawn workflow, obligation seams, conformance helpers. | Process lifecycle APIs, direct concrete runtime dependencies. |
| `t3claw_engine` | `t3claw_engine/AGENTS.md`, `t3claw_engine/CLAUDE.md`, `t3claw_engine/MONTY.md` | Thread/capability/CodeAct engine: runtime manager, executor, gates, leases, memory retrieval, workspace mounts, traits/types. | Product transport, provider-specific auth, lower-layer host policy shortcuts. |

### Product, adapters, Reborn binary

| Crate | Load first | Owns / go here for | Avoid moving in |
| --- | --- | --- | --- |
| `t3claw_reborn` | `t3claw_reborn/AGENTS.md`, `t3claw_reborn/CLAUDE.md` | Standalone Reborn composition/adapters: driver registry, home/profile/doctor support, runtime composition seams. | V1 root runtime imports unless explicitly bridged. |
| `t3claw_reborn_config` | `t3claw_reborn_config/AGENTS.md`, `Cargo.toml`, `src/lib.rs` | Boot configuration contracts for standalone Reborn binary. | Runtime execution or product adapter behavior. |
| `t3claw_reborn_composition` | `t3claw_reborn_composition/AGENTS.md`, `t3claw_reborn_composition/CLAUDE.md` | Facade-shaped production composition root for Reborn. | Low-level policy internals that belong to service crates. |
| `t3claw_reborn_openai_compat` | `t3claw_reborn_openai_compat/AGENTS.md`, `t3claw_reborn_openai_compat/CLAUDE.md` | Reborn-native OpenAI-compatible API route descriptors, Chat/Responses DTOs, sanitized error envelope, and fail-closed route fragment. | V1 gateway handlers, direct LLM proxying, listener binding, ProductWorkflow internals/direct runtime wiring. |
| `t3claw_reborn_openai_compat_storage` | `t3claw_reborn_openai_compat_storage/AGENTS.md`, `t3claw_reborn_openai_compat_storage/CLAUDE.md` | Durable storage adapters for Reborn OpenAI-compatible public refs and idempotency mappings. | HTTP route handlers, ProductWorkflow orchestration, v1 gateway handlers, direct LLM proxying. |
| `t3claw_first_party_extensions` | `t3claw_first_party_extensions/AGENTS.md`, `Cargo.toml` | Concrete first-party userland extension implementations and deterministic tool behavior behind scoped handles. | Host runtime composition, loop-facing ports, ambient runtime authority, dispatcher/network/secrets handles. |
| `t3claw_first_party_extension_ports` | `t3claw_first_party_extension_ports/AGENTS.md`, `Cargo.toml` | Loop-facing adapters for first-party extensions: skill activation/context/execution ports over loop-support and turn-run contracts. | Concrete tool behavior, host runtime composition, product workflow, raw host authority. |
| `t3claw_reborn_cli` | `t3claw_reborn_cli/AGENTS.md` | Standalone Reborn CLI, command files, CLI context, shell completions, doctor/home/profile commands. | V1 runtime imports, root `t3claw` deps, side effects in pure commands. |
| `t3claw_product_adapters` | `t3claw_product_adapters/AGENTS.md`, `t3claw_product_adapters/CLAUDE.md` | Product-adapter contracts: adapter trait, auth, egress, identity, workflow, external/projection/inbound, redaction, fakes. | Host runtime internals or specific WASM runner implementation. |
| `t3claw_product_adapter_registry` | `t3claw_product_adapter_registry/AGENTS.md`, `t3claw_product_adapter_registry/CLAUDE.md` | ProductAdapter host-api projection and installation registry. | Adapter execution or product workflow orchestration. |
| `t3claw_product_workflow` | `t3claw_product_workflow/AGENTS.md`, `t3claw_product_workflow/CLAUDE.md` | Product-facing workflow facade: inbound turns, bindings, ledger, workflow/errors, Reborn service bridges. | Low-level runtime lane internals or direct provider-specific transports. |
| `t3claw_product_workflow_storage` | `t3claw_product_workflow_storage/AGENTS.md`, `Cargo.toml` | Durable libSQL/PostgreSQL adapters for the product workflow idempotency ledger. | Workflow orchestration, direct dispatch, or divergence between libSQL and PostgreSQL behavior. |
| `t3claw_wasm_product_adapters` | `t3claw_wasm_product_adapters/AGENTS.md`, `t3claw_wasm_product_adapters/CLAUDE.md` | WASM v2 ProductAdapter runtime: component runner, egress policy, auth verifier, bindings, store. | Generic WASM lane semantics or product workflow decisions. |
| `t3claw_telegram_v2_adapter` | `t3claw_telegram_v2_adapter/AGENTS.md`, `Cargo.toml`, `src/lib.rs` | Telegram WASM v2 ProductAdapter tracer bullet: payload parsing, rendering, adapter implementation. | Shared adapter contracts or registry semantics. |
| `t3claw_reborn_webui_ingress` | `t3claw_reborn_webui_ingress/AGENTS.md`, `Cargo.toml` | Host-owned listener binding, authenticator implementations, and serve loop for Reborn WebChat v2. | Product/API route semantics, transcript storage, v1 channel code, product adapter transport shims. |

### LLM, skills, safety, UI, helpers

| Crate | Load first | Owns / go here for | Avoid moving in |
| --- | --- | --- | --- |
| `t3claw_llm` | `t3claw_llm/AGENTS.md`, `t3claw_llm/CLAUDE.md`, `t3claw_llm/Cargo.toml` | Multi-provider LLM integration: provider trait, auth, registry, retry/failover/circuit breaker/cache, tool schemas, reasoning, tracing, transcription/vision. | Engine loop ownership or product workflow. |
| `t3claw_skills` | `t3claw_skills/AGENTS.md` | Skill catalog, parser, gating, selector/scoring, registry, validation, v2 skill types. | Agent-loop execution or UI command routing. |
| `t3claw_safety` | `t3claw_safety/AGENTS.md`, `crates/t3claw_safety/fuzz/README.md` | Prompt-injection detection, validation, sanitization, safety policy, sensitive paths, credential detection, leak scanning, fuzz/benches. | Sandbox execution, credential storage/injection, network allowlists, dispatch, UI decisions. |
| `t3claw_gateway` | `t3claw_gateway/AGENTS.md` | Gateway frontend assets, layout config, bundle metadata, widget extension system. | Browser API/web channel runtime (`src/channels/web/`) or product workflow. |
| `t3claw_webui_v2` | `t3claw_webui_v2/AGENTS.md`, `t3claw_webui_v2/CLAUDE.md` | Reborn WebChat v2 route descriptors, axum handlers, schemas, and redacted HTTP error shape behind `webui-v2-beta`. | Bearer validation, CSRF/origin/rate-limit middleware, direct runtime/DB access, unredacted responses. |
| `t3claw_tui` | `t3claw_tui/AGENTS.md`, `t3claw_tui/CLAUDE.md` | Ratatui app, widgets, layout, render, theme, event/input loop, spinner. | Main crate channel bridge (`src/channels/tui.rs`) or backend workflow. |
| `t3claw_silk_decoder` | `t3claw_silk_decoder/AGENTS.md`, `t3claw_silk_decoder/README.md`, `t3claw_silk_decoder/Cargo.toml`, `t3claw_silk_decoder/src/main.rs` | Excluded helper binary that decodes WeChat SILK v3 voice notes to WAV. | Main workspace build dependencies; keep libclang isolated. |

## Common Change Routes

- Host API shape: `t3claw_host_api` -> matching `docs/reborn/contracts/*.md` -> affected service/runtime crates -> `t3claw_architecture`.
- Storage and persistence: owning domain crate for schemas/queries; preserve libSQL/PostgreSQL parity where applicable. Product workflow ledger adapters live in `t3claw_product_workflow_storage`; event/audit store backends live in `t3claw_reborn_event_store`.
- Files/memory: `t3claw_filesystem` for mount/path authority; `t3claw_memory` for memory documents/search/chunking/indexing.
- Events/projections/outbound: `t3claw_events` for canonical redacted events; `t3claw_event_projections` for projection model; `t3claw_event_streams` for transport-neutral live/replay streams; `t3claw_outbound` for metadata-only delivery/subscription policy; adapters for concrete delivery.
- Trust/auth/approval: `t3claw_trust` -> `t3claw_authorization` -> `t3claw_run_state`/`t3claw_approvals` -> `t3claw_capabilities` as needed.
- Hooks and prompt context: `t3claw_hooks` for hook registration/dispatch/failure policy; `t3claw_prompt_envelope` for model-visible untrusted or trust-labeled snippet wrapping.
- Runtime execution: lane crate (`scripts`, `mcp`, `wasm`) first; `dispatcher` for routing; `host_runtime` for secrets/network/resources/redaction; `processes` for background lifecycle; `t3claw_wasm_limiter` only for shared limiter mechanics.
- Turns/agent loop: `t3claw_turns` for turn coordination; `t3claw_agent_loop` for strategy/planner/executor contracts; `t3claw_loop_support` for host support ports; `t3claw_engine` for CodeAct/thread runtime.
- Product adapter flow: `t3claw_product_adapters` contracts -> `t3claw_product_adapter_registry` installation/projection -> `t3claw_product_workflow` orchestration -> concrete adapter crate.
- Reborn binary/composition: `t3claw_reborn_config` for boot config; `t3claw_reborn_composition` for production wiring; `t3claw_reborn_cli` for commands; `t3claw_reborn` for standalone adapters/driver registry; `t3claw_reborn_webui_ingress` for host-owned WebChat v2 listener lifecycle.
- Model/provider behavior: `t3claw_llm`; do not leak provider auth/cache/retry concerns into engine or product workflow.
- UI presentation: `t3claw_tui`, `t3claw_gateway`, or `t3claw_webui_v2`; backend API/web channel code remains under root `src/` unless the surface is the Reborn WebChat v2 route crate.

## Testing

Prefer narrow tests during iteration:

```bash
cargo test -p t3claw_host_api
cargo test -p t3claw_network network_policy_contract
cargo test -p t3claw_outbound --all-features
cargo test -p t3claw_product_workflow
cargo test -p t3claw_wasm --test wit_tool_runtime_contract
```

Then expand by risk:

```bash
cargo test -p t3claw_architecture
scripts/check-boundaries.sh
scripts/reborn-e2e-rust.sh
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Persistence behavior must support PostgreSQL and libSQL where applicable. If local Postgres is unavailable, follow crate-local skip flags only when docs/tests explicitly permit them.

## Guardrails

- Avoid `.unwrap()` / `.expect()` in production; use typed errors with context.
- Preserve tenant/user/agent/project/mission/thread scope on authority, state, memory, process, network, outbound, resource, and event records.
- Fail closed for auth, approvals, trust, filesystem containment, network policy, secret leases, runtime selection, and adapter identity.
- Do not expose raw secrets, backend paths, private URLs, transport internals, raw SQL/backend errors, or unredacted runtime/user content across public surfaces.
- Keep runtime crates untrusted: host-runtime mediates secrets/network/redaction/accounting.
- Keep declarative crates declarative: manifests, contracts, registries, and policy descriptions should not perform execution side effects.
- Use existing traits/ports/registries; avoid hardcoded cross-crate shortcuts.
- Test through caller when a helper gates dispatch, persistence, network, secrets, approvals, resources, events, process, adapter, or UI side effects.

## Docs / Parity Checklist

Behavior changes may require updates to:

- crate-local `AGENTS.md`, `CLAUDE.md`, `CONTRACT.md`, or `README.md`
- `docs/reborn/contracts/*.md`
- `FEATURE_PARITY.md`
- crate changelogs for packages that publish independently
- architecture boundary tests in `crates/t3claw_architecture`
