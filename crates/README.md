# T3Claw crates

This directory contains the Rust crates that split T3Claw into smaller, reviewable boundaries. Most crates are Reborn system-service crates: they hold one slice of host authority, storage, policy, runtime composition, or product UI glue.

Use this page as a human map before opening individual crate docs or source files.

## Mental model

T3Claw Reborn keeps authority narrow and explicit:

1. **Contracts describe authority**: `t3claw_host_api` and adjacent contract crates define scoped identities, policies, requests, decisions, and DTOs.
2. Policy gates decide: authorization, trust, runtime policy, resources, approvals, secrets, safety, filesystem, network, hooks, and prompt-envelope crates each own one kind of decision, label, or side effect.
3. Capability hosts coordinate: capabilities, dispatcher, processes, scripts, MCP, WASM, shared WASM limiting, and host-runtime crates compose validated requests into sandboxed execution.
4. State is durable and replayable: events, event streams, run state, threads, conversations, memory, outbound, product workflow storage, traces, and event projections keep the host observable without leaking secrets.
5. Product surfaces adapt: agent loop, engine, loop support, gateway, WebChat v2, TUI, skills, first-party extensions, product adapters, and Reborn composition crates turn those lower-level boundaries into agent and user experiences.

A good rule of thumb: if a change adds new authority or persistence, put it in the crate that owns that boundary instead of threading it through a UI or runtime crate.

## Crate groups

### Core vocabulary and shared contracts

| Crate directory | Package | Human context |
| --- | --- | --- |
| `t3claw_common` | `t3claw_common` | Shared workspace types and utilities that are not authority-bearing enough to belong in `t3claw_host_api`. Keep this small. |
| `t3claw_host_api` | `t3claw_host_api` | Canonical Reborn authority vocabulary: actors, scopes, policies, capability requests, decisions, obligations, and host-facing data contracts. Runtime behavior belongs elsewhere. |
| `t3claw_prompt_envelope` | `t3claw_prompt_envelope` | Leaf helper for wrapping trusted or untrusted prompt snippets with closed-vocabulary source/trust labels and rejecting instruction-hijack markers. |
| `t3claw_runtime_policy` | `t3claw_runtime_policy` | Resolves runtime profiles from host configuration and policy inputs. Use it when choosing what runtime shape a capability may use. |
| `t3claw_architecture` | `t3claw_architecture` | Workspace architecture contract tests. It has no production role; it fails builds when crate dependency boundaries drift. |

### Authority, safety, and policy gates

| Crate directory | Package | Human context |
| --- | --- | --- |
| `t3claw_authorization` | `t3claw_authorization` | Evaluates host API authority contracts before capability execution. It should not execute work, reserve resources, or prompt users. |
| `t3claw_approvals` | `t3claw_approvals` | Resolves durable approval requests and issues scoped authorization leases. It does not own prompting UI or runtime execution. |
| `t3claw_trust` | `t3claw_trust` | Host-controlled trust-class policy engine. Use it for decisions about how much trust a runtime, extension, or input receives. |
| `t3claw_resources` | `t3claw_resources` | Resource reservation governor. Owns budget/reservation mechanics, not runtime dispatch. |
| `t3claw_auth` | `t3claw_auth` | Product-facing Reborn auth setup contracts: auth-flow records, secure manual-token interactions, credential accounts, provider exchange, continuations, cleanup, and fakes. |
| `t3claw_safety` | `t3claw_safety` | Prompt-injection defense, input validation, secret-leak detection, and safety policy enforcement. |
| `t3claw_secrets` | `t3claw_secrets` | Tenant-scoped secret storage and leasing behind opaque `SecretHandle` values. It stores/leases material; other crates decide when leases are allowed and where to inject them. |
| `t3claw_network` | `t3claw_network` | Network policy and HTTP egress boundary. Resolves DNS, rejects disallowed/private targets when configured, and owns host-mediated outbound HTTP. |
| `t3claw_filesystem` | `t3claw_filesystem` | Scoped filesystem service. Use it for host-controlled path access, not direct runtime path handling. |
| `t3claw_hooks` | `t3claw_hooks` | Reborn loop hook framework. Owns trust-tiered hook contracts, predicates, ordering, dispatch, and failure policy; hooks cannot grant authority. |

### Capability execution and runtime lanes

| Crate directory | Package | Human context |
| --- | --- | --- |
| `t3claw_capabilities` | `t3claw_capabilities` | Caller-facing capability invocation host. Coordinates authorization, approvals, run-state transitions, and neutral runtime dispatch. |
| `t3claw_dispatcher` | `t3claw_dispatcher` | Composition-only runtime dispatch contracts. Wires validated extension descriptors to runtime lanes; it does not parse manifests or grant authority. |
| `t3claw_processes` | `t3claw_processes` | Host-tracked background process lifecycle. Owns lifecycle mechanics, not capability policy. |
| `t3claw_scripts` | `t3claw_scripts` | Script/CLI capability runner contracts. Executes declared commands through a host-selected backend. |
| `t3claw_mcp` | `t3claw_mcp` | Adapts manifest-declared MCP tools into T3Claw capabilities without granting ambient filesystem, secret, or network authority. |
| `t3claw_wasm` | `t3claw_wasm` | Reborn WASM component runtime lane. Owns component-model/WIT runtime surface and sandboxed WASM execution details. |
| `t3claw_wasm_limiter` | `t3claw_wasm_limiter` | Shared `wasmtime::ResourceLimiter` used by WASM tool and hook runtimes so memory/table/instance limits do not drift. |
| `t3claw_wasm_sandbox_core` | `t3claw_wasm_sandbox_core` | Shared WASM sandbox primitives used below product adapters and runtime lanes. |
| `t3claw_wasm_product_adapters` | `t3claw_wasm_product_adapters` | WASM-side adapters that bridge guest components into product-facing shapes. Keeps host-only authority out of the guest. |
| `t3claw_extensions` | `t3claw_extensions` | Extension manifest, lifecycle, and registration contracts. Owns install/activate/remove semantics; runtime crates consume validated descriptors from here. |
| `t3claw_host_runtime` | `t3claw_host_runtime` | Narrow facade upper Reborn services depend on. Provides `HostRuntime` plus production composition around capability hosting. |

### Durable state, eventing, and read models

| Crate directory | Package | Human context |
| --- | --- | --- |
| `t3claw_events` | `t3claw_events` | Redacted runtime/audit vocabulary plus durable append-log traits. Use it for observable history, not current state. |
| `t3claw_reborn_event_store` | `t3claw_reborn_event_store` | Concrete Reborn event/audit store backends and backend-profile validation. Depends on `t3claw_events`; keeps storage adapters out of event vocabulary. |
| `t3claw_event_projections` | `t3claw_event_projections` | Product-facing read models over durable runtime and audit logs. Upper layers should consume these DTOs rather than parse event rows directly. |
| `t3claw_event_streams` | `t3claw_event_streams` | Transport-neutral projection stream manager: admission, bounded subscription buffers, replay/live updates, lag signals, and redaction validation. |
| `t3claw_run_state` | `t3claw_run_state` | Current lifecycle state for host-managed invocations. Events are history; run state answers “what is happening now?” |
| `t3claw_threads` | `t3claw_threads` | Canonical session thread and transcript service contracts. Use it for durable thread/transcript ownership. |
| `t3claw_conversations` | `t3claw_conversations` | Conversation binding and session-thread contracts that connect product conversation concepts to Reborn threads. |
| `t3claw_memory` | `t3claw_memory` | Memory document service adapters. This is for workspace/memory document semantics, not arbitrary transcript deletion. |
| `t3claw_outbound` | `t3claw_outbound` | Metadata-only outbound state: notification policy, projection subscription cursors, and delivery status. It does not own transport delivery or payload content. |
| `t3claw_reborn_traces` | `t3claw_reborn_traces` | Trace Commons / TraceDAO client surface: contribution pipeline, trace client, redaction helpers, and conversation-message compatibility type. |

### Product, agent loop, and user surfaces

| Crate directory | Package | Human context |
| --- | --- | --- |
| `t3claw_reborn` | `t3claw_reborn` | Standalone Reborn composition and adapters. This is the high-level Reborn composition crate. |
| `t3claw_reborn_composition` | `t3claw_reborn_composition` | Wiring layer that assembles Reborn services into the host runtime. Composition-only; no policy or persistence logic of its own. |
| `t3claw_reborn_config` | `t3claw_reborn_config` | Reborn boot-config boundary: typed configuration, profiles, and validation consumed before services start. |
| `t3claw_reborn_cli` | `t3claw_reborn_cli` | Reborn-first CLI surface (command modules, completion, shell entry points). Calls into composition; does not own host policy. |
| `t3claw_reborn_webui_ingress` | `t3claw_reborn_webui_ingress` | Host-owned listener binding, authenticator implementations, and serve loop for the Reborn WebChat v2 HTTP gateway. |
| `t3claw_reborn_openai_compat_storage` | `t3claw_reborn_openai_compat_storage` | Durable filesystem-backed storage adapters for Reborn OpenAI-compatible public refs and idempotency mappings. |
| `t3claw_llm` | `t3claw_llm` | LLM provider routing and abstraction used by Reborn product surfaces and the agent loop. |
| `t3claw_agent_loop` | `t3claw_agent_loop` | Agent-loop framework state, planner/executor, strategy/family contracts, and test support. |
| `t3claw_loop_support` | `t3claw_loop_support` | Adapts durable Reborn support boundaries into the narrow agent-loop host port. It should not own provider clients or runtime dispatchers. |
| `t3claw_turns` | `t3claw_turns` | Host-layer turn coordination contracts. Use it for turn lifecycle boundaries between loop/product code and host services. |
| `t3claw_first_party_extensions` | `t3claw_first_party_extensions` | Concrete first-party userland extension implementations behind scoped handles. |
| `t3claw_first_party_extension_ports` | `t3claw_first_party_extension_ports` | Loop-facing adapters for first-party extensions: skill activation/context/execution ports over loop-support and turn-run contracts. |
| `t3claw_product_adapters` | `t3claw_product_adapters` | Product-adapter contracts for mapping Reborn state and events into product-facing shapes. |
| `t3claw_product_adapter_registry` | `t3claw_product_adapter_registry` | ProductAdapter host-api projection and installation registry. |
| `t3claw_product_workflow` | `t3claw_product_workflow` | Product-facing workflow facade: inbound turn service, idempotency ledger, binding resolution. |
| `t3claw_product_workflow_storage` | `t3claw_product_workflow_storage` | Durable libSQL/PostgreSQL adapters for the product workflow idempotency ledger. |
| `t3claw_engine` | `t3claw_engine` | Unified thread-capability-CodeAct execution engine. It is closer to product/agent orchestration than low-level host policy. |
| `t3claw_skills` | `t3claw_skills` | Skill selection, scoring, and management. |
| `t3claw_gateway` | `t3claw_gateway` | Browser gateway frontend assets, layout configuration, and widget extension system. |
| `t3claw_webui_v2` | `t3claw_webui_v2` | Reborn WebChat v2 HTTP route surface and route descriptors. Off by default; enable with `webui-v2-beta`. |
| `t3claw_tui` | `t3claw_tui` | Modular Ratatui-based terminal UI. |
| `t3claw_telegram_v2_adapter` | `t3claw_telegram_v2_adapter` | Telegram v2 channel adapter for the Reborn product surface. Maps Telegram traffic into Reborn capability and turn contracts. |
| `t3claw_silk_decoder` | `t3claw_silk_decoder` | Standalone WeChat `audio/silk` decoder helper. Excluded from the default workspace build; needs `libclang` and a C toolchain. |

## Where to make common changes

- **New capability type or host API contract**: start in `t3claw_host_api`, then update authorization/capability/runtime crates that consume it.
- **Authorization or approval behavior**: use `t3claw_authorization` for policy decisions and `t3claw_approvals` for approval lease resolution.
- **Secret storage or leasing**: use `t3claw_secrets`; do not put SQL or crypto details in engine, gateway, or runtime lanes.
- **Network or filesystem access**: use `t3claw_network` or `t3claw_filesystem`; runtimes should ask host services instead of bypassing them.
- **WASM, MCP, or script execution**: use the corresponding runtime-lane crate plus `t3claw_capabilities`/`t3claw_dispatcher` for coordination.
- **Hook behavior or prompt snippet trust labeling**: use `t3claw_hooks` for hook contracts/dispatch and `t3claw_prompt_envelope` for model-facing snippet wrapping.
- **Extension lifecycle (install/activate/remove)**: use `t3claw_extensions`; do not parse manifests or reimplement registration in runtime or UI crates.
- **Reborn composition or boot config**: use `t3claw_reborn_composition` and `t3claw_reborn_config`; keep `main.rs`/CLI entry points thin.
- **LLM provider routing**: use `t3claw_llm`; do not wire provider clients directly into engine or gateway crates.
- **Channel adapters (e.g., Telegram)**: use the channel adapter crate (`t3claw_telegram_v2_adapter`); keep authority in lower host crates.
- **Durable event history**: use `t3claw_events` for contracts and `t3claw_reborn_event_store` for backend adapters.
- **Current invocation state**: use `t3claw_run_state`, not event logs.
- **User-visible read models and live projection streams**: prefer `t3claw_event_projections`, `t3claw_event_streams`, or `t3claw_product_adapters` over parsing storage rows in UI code.
- **Product workflow persistence**: keep orchestration in `t3claw_product_workflow` and durable ledger adapters in `t3claw_product_workflow_storage`.
- **Agent loop/product orchestration**: use `t3claw_agent_loop`, `t3claw_loop_support`, `t3claw_turns`, `t3claw_engine`, or `t3claw_reborn` depending on layer.
- **Web or terminal UI**: use `t3claw_gateway`, `t3claw_webui_v2`, `t3claw_reborn_webui_ingress`, or `t3claw_tui`; keep authority and persistence in lower crates.

## Boundary rules

- Keep crate-owned logic in the owning crate. Avoid reimplementing module-specific setup in `src/main.rs`, `src/app.rs`, gateway, or TUI code.
- Prefer extending existing traits and service boundaries over adding one-off integration paths.
- Do not give runtime lanes ambient access to secrets, filesystem, network, or process control. Route through host services.
- Treat `t3claw_host_api` as the shared contract layer. It may define authority-bearing shapes; it should not perform side effects.
- Use `t3claw_architecture` tests when dependency boundaries need to become enforceable.
- If behavior changes, check `../CLAUDE.md`, `../AGENTS.md`, and `../FEATURE_PARITY.md` for test/doc update expectations.

## Quick commands

From the repository root:

```bash
cargo fmt
cargo clippy --all --benches --tests --examples --all-features
cargo test
```

For targeted crate work, prefer the narrowest command first:

```bash
cargo test -p t3claw_secrets --features libsql
cargo clippy -p t3claw_network --tests -- -D warnings
```

Some crates are feature-gated or test backends conditionally. Read the crate-level docs and tests before assuming a command covers PostgreSQL, libSQL, WASM, or integration behavior.
