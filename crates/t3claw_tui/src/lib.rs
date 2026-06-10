//! `t3claw_tui` — Modular Ratatui-based TUI for T3Claw.
//!
//! This crate provides the rendering engine, widget system, and event loop
//! for T3Claw's terminal user interface. It is intentionally decoupled
//! from the main `t3claw` crate: the Channel trait bridge lives in
//! `src/channels/tui.rs` in the main crate.
//!
//! # Architecture
//!
//! ```text
//! ┌─ TuiApp (app.rs) ────────────────────────────────────────────┐
//! │  Event loop: poll crossterm → merge with TuiEvent rx         │
//! │  Render: Layout → Widget::render() → Terminal::draw()        │
//! │                                                              │
//! │  ┌─ Header ─────────────────────────────────────────────┐    │
//! │  │  version · model · duration · tokens                 │    │
//! │  ├─ Conversation ──────────┬─ Sidebar ──────────────────┤    │
//! │  │  Messages + markdown    │  Tools: live activity      │    │
//! │  │                         │  Threads: active/recent    │    │
//! │  ├─ Input ─────────────────┴────────────────────────────┤    │
//! │  │  › user input (tui-textarea)                         │    │
//! │  ├─ Status Bar ─────────────────────────────────────────┤    │
//! │  │  model │ tokens │ cost │ keybinds                    │    │
//! │  └──────────────────────────────────────────────────────┘    │
//! └──────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Communication
//!
//! The main crate sends [`TuiEvent`]s via the handle's `event_tx`, and
//! receives user messages via `msg_rx`. The TUI never calls into the
//! main crate directly.
#![warn(unreachable_pub)]

mod app;
mod event;
mod input;
mod layout;
mod render;
mod spinner;
mod theme;
mod widgets;

pub use app::{TuiAppConfig, TuiAppHandle, start_tui};
pub use event::{
    EngineThreadDetailEntry, EngineThreadEntry, EngineThreadMessageEntry, HistoryApprovalRequest,
    HistoryMessage, ThreadEntry, TuiEvent, TuiUiAction, TuiUserMessage,
};
pub use layout::TuiLayout;
pub use widgets::{SkillCategory, ToolCategory};
