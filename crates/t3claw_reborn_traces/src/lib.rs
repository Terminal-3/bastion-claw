//! Trace Commons / TraceDAO client extracted from the T3Claw monolith.
//!
//! This crate holds the trace contribution pipeline (`contribution`), the
//! host-facing trace client (`client`), the redaction helpers used to scrub
//! sensitive JSON before submission (`redaction`), and the shared
//! `ConversationMessage` type that the legacy monolith's `history` module now
//! re-exports for backward compatibility.

pub mod client;
pub mod contribution;
pub mod conversation_message;
pub mod redaction;

pub use conversation_message::ConversationMessage;

/// Recorded-trace deserialization surface for callers that load JSON traces
/// off disk (e.g. `t3claw-reborn traces preview`). Re-exports from
/// `t3claw_llm::recording` so reborn-cli does not need a direct
/// `t3claw_llm` dependency, preserving the architectural boundary.
pub mod recording {
    pub use t3claw_llm::recording::*;
}

/// Filesystem path resolution for trace-contribution storage. Re-exports
/// from `t3claw_common::paths` so reborn-cli does not need a direct
/// `t3claw_common` dependency.
pub mod paths {
    pub use t3claw_common::paths::*;
}
