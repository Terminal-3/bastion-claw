use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use t3claw_event_projections::{
    CapabilityActivityProjection, CapabilityActivityStatus, EventProjectionService,
    ProjectionCursor, ProjectionError, ProjectionReplay, ProjectionRequest, ProjectionScope,
    ProjectionSnapshot, RunProjectionStatus, RunStatusProjection, ThreadTimeline, TimelineEntry,
    TimelineEntryKind,
};
use t3claw_event_streams::{
    AllowAllProjectionAccessPolicy, EventStreamManager, InMemoryProjectionStreamAdmissionPolicy,
    InMemoryProjectionUpdateSource, LagReason, NoExposureProjectionRedactionValidator,
    ProductProjectionEnvelope, ProjectionAccessPolicy, ProjectionAccessRequest,
    ProjectionFetchRequest, ProjectionLiveUpdateRequest, ProjectionRedactionValidator,
    ProjectionStreamAdmissionPolicy, ProjectionStreamAdmissionRequest, ProjectionStreamError,
    ProjectionStreamItem, ProjectionStreamLimits, ProjectionSubscribeRequest, ProjectionTarget,
    ProjectionUpdateSource, ProjectionViewClass, PushCandidatesForUpdateRequest,
    SubscriberCapabilities, keep_alive_item,
};
use t3claw_events::{EventCursor, EventStreamKey, ReadScope};
use t3claw_host_api::{
    CapabilityId, ExtensionId, InvocationId, MissionId, ProjectId, RuntimeKind, TenantId, ThreadId,
    UserId,
};
use t3claw_outbound::{
    AdvanceSubscriptionCursorRequest, InMemoryOutboundStateStore, LoadSubscriptionCursorRequest,
    OutboundDeliveryAttempt, OutboundError, OutboundPushKind, OutboundPushPlan,
    OutboundPushTargetRequest, OutboundStateStore, ProjectionSubscriptionRecord,
    ProjectionUpdateRef, ThreadNotificationPolicy, ThreadNotificationTarget,
    UpdateDeliveryStatusRequest,
};
use t3claw_turns::{ReplyTargetBindingRef, TurnActor, TurnScope};
use tokio::{
    sync::Barrier,
    time::{Duration, timeout},
};
