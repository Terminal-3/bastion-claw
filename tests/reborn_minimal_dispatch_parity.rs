#[allow(dead_code)]
#[path = "support/reborn/mod.rs"]
mod reborn_support;
mod support;

use reborn_support::harness::RebornBinaryE2EHarness;
use t3claw_product_adapters::ProductInboundAck;
use t3claw_threads::{MessageKind, MessageStatus};
use t3claw_turns::TurnStatus;

#[tokio::test]
async fn reborn_minimal_dispatch_parity() {
    let mut harness =
        RebornBinaryE2EHarness::reply_only("room-minimal-dispatch", "minimal dispatch complete")
            .await
            .expect("harness");
    harness.start();

    let submitted = harness
        .submit_text("event-minimal-dispatch", "ping")
        .await
        .expect("submit text");
    assert!(matches!(submitted.ack, ProductInboundAck::Accepted { .. }));

    harness
        .wait_for_status(submitted.run_id, TurnStatus::Completed)
        .await
        .expect("completed run");
    harness
        .assert_final_reply("minimal dispatch complete")
        .await
        .expect("final reply");
    let history = harness.history().await.expect("thread history");
    assert!(
        history
            .iter()
            .any(|message| message.kind == MessageKind::User
                && message.status == MessageStatus::Submitted
                && message.content.as_deref() == Some("ping")),
        "history accessor should expose the submitted inbound message"
    );
    assert!(
        history
            .iter()
            .any(|message| message.kind == MessageKind::Assistant
                && message.status == MessageStatus::Finalized
                && message.content.as_deref() == Some("minimal dispatch complete")),
        "history accessor should expose the finalized assistant reply"
    );
    assert_eq!(harness.model_requests().len(), 1);

    harness.shutdown().await;
}
