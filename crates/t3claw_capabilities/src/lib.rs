//! Capability invocation host contracts for T3Claw Reborn.
//!
//! `t3claw_capabilities` is the caller-facing capability invocation service.
//! It coordinates authorization, approval resume, run-state transitions, and
//! neutral runtime dispatch without depending on concrete runtime crates.
#![warn(unreachable_pub)]

mod conformance;
mod error;
mod helpers;
mod host;
mod obligations;
mod requests;

pub use conformance::{
    CapabilityProfileClaim, CapabilityProfileClaimedOperation, CapabilityProfileConformanceFinding,
    CapabilityProfileConformanceFindingKind, CapabilityProfileConformanceReport,
    evaluate_profile_conformance,
};
pub use error::{CapabilityInvocationError, ResumeContextMismatchKind};
pub use host::CapabilityHost;
pub use obligations::{
    CapabilityObligationAbortRequest, CapabilityObligationCompletionRequest,
    CapabilityObligationError, CapabilityObligationFailureKind, CapabilityObligationHandler,
    CapabilityObligationOutcome, CapabilityObligationPhase, CapabilityObligationRequest,
};
pub use requests::{
    CapabilityInvocationRequest, CapabilityInvocationResult, CapabilityResumeRequest,
    CapabilitySpawnRequest, CapabilitySpawnResult,
};
