//! Desktop notification request shaping (host port; no D-Bus in core).
//!
//! The app shapes generation-done / generation-failed intents here. Linux
//! backends (Notification portal / freedesktop) live outside `ronin_core`.

use std::sync::Mutex;

use crate::trust::{may_inject_into_chat_request, scrub_ambient_payload, ContextOrigin};

/// Portal / freedesktop action id that focuses the thread that finished.
pub const FOCUS_THREAD_ACTION: &str = "focus-thread";

/// Stable notification id prefix for generation events.
pub const GENERATION_NOTIFICATION_ID_PREFIX: &str = "chat.ronin.generation";

/// Errors from a [`DesktopNotifier`] backend.
#[derive(Debug, thiserror::Error)]
pub enum NotificationError {
    /// Backend could not deliver the notification.
    #[error("desktop notification failed: {0}")]
    DeliveryFailed(String),
}

/// Whether a generation finished successfully or failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerationNotifyKind {
    /// Assistant stream completed.
    Completed,
    /// Assistant stream failed.
    Failed,
}

/// Inputs for shaping a generation notification (pre-scrub, pre-disable gate).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerationNotifyInput {
    /// Completed vs failed.
    pub kind: GenerationNotifyKind,
    /// Thread that finished generating.
    pub thread_id: String,
    /// Optional human title for the body.
    pub thread_title: Option<String>,
    /// Optional failure summary (failed only; scrubbed before delivery).
    pub error_summary: Option<String>,
}

/// User preference gate for desktop notifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotificationPrefs {
    /// When false, shaping returns [`None`] (no delivery).
    pub enabled: bool,
}

impl Default for NotificationPrefs {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// A single action button offered on the notification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationButton {
    /// Action id invoked when the button is pressed.
    pub action_id: String,
    /// User-visible label.
    pub label: String,
}

/// Shaped desktop notification ready for a host backend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopNotificationRequest {
    /// Stable id for replace/withdraw.
    pub id: String,
    /// Notification title (secrets scrubbed).
    pub title: String,
    /// Notification body (secrets scrubbed).
    pub body: String,
    /// Default activation action when supported (e.g. focus thread).
    pub default_action: Option<String>,
    /// Optional action buttons.
    pub buttons: Vec<NotificationButton>,
    /// Thread this notification refers to.
    pub thread_id: String,
    /// Completed vs failed.
    pub kind: GenerationNotifyKind,
}

/// Thin host port: deliver a shaped notification (portal / freedesktop / test double).
pub trait DesktopNotifier {
    /// Deliver `request` to the desktop (or record it in tests).
    fn notify(&self, request: &DesktopNotificationRequest) -> Result<(), NotificationError>;

    /// Drain one pending focus/open-thread action, if the backend supports it.
    fn poll_focus_thread(&self) -> Option<String> {
        None
    }
}

/// Test double that records every notify call.
#[derive(Debug, Default)]
pub struct RecordingDesktopNotifier {
    sent: Mutex<Vec<DesktopNotificationRequest>>,
}

impl RecordingDesktopNotifier {
    /// Creates an empty recorder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Snapshot of delivered requests (oldest first).
    pub fn take_sent(&self) -> Vec<DesktopNotificationRequest> {
        std::mem::take(&mut *self.sent.lock().unwrap_or_else(|p| p.into_inner()))
    }

    /// Number of recorded deliveries.
    pub fn len(&self) -> usize {
        self.sent.lock().unwrap_or_else(|p| p.into_inner()).len()
    }

    /// Whether no deliveries were recorded.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl DesktopNotifier for RecordingDesktopNotifier {
    fn notify(&self, request: &DesktopNotificationRequest) -> Result<(), NotificationError> {
        self.sent
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .push(request.clone());
        Ok(())
    }
}

/// No-op notifier (quiet desktops / tests that ignore delivery).
#[derive(Debug, Default, Clone, Copy)]
pub struct NullDesktopNotifier;

impl DesktopNotifier for NullDesktopNotifier {
    fn notify(&self, _request: &DesktopNotificationRequest) -> Result<(), NotificationError> {
        Ok(())
    }
}

/// Context origin for notification title/body — never model context.
pub fn notification_payload_origin() -> ContextOrigin {
    ContextOrigin::NotificationPayload
}

/// Whether notification payload text may merge into a provider chat request.
pub fn notification_may_inject_into_chat_request() -> bool {
    may_inject_into_chat_request(notification_payload_origin())
}

/// Shape a generation-done / generation-failed notification.
///
/// Returns [`None`] when notifications are disabled. Title and body always pass
/// through [`scrub_ambient_payload`]. Includes a focus-thread default action.
pub fn shape_generation_notification(
    prefs: &NotificationPrefs,
    input: &GenerationNotifyInput,
) -> Option<DesktopNotificationRequest> {
    if !prefs.enabled {
        return None;
    }
    if input.thread_id.trim().is_empty() {
        return None;
    }

    let (raw_title, raw_body) = match input.kind {
        GenerationNotifyKind::Completed => (
            "Ronin — generation complete".to_string(),
            completed_body(input.thread_title.as_deref()),
        ),
        GenerationNotifyKind::Failed => (
            "Ronin — generation failed".to_string(),
            failed_body(
                input.thread_title.as_deref(),
                input.error_summary.as_deref(),
            ),
        ),
    };

    let title = scrub_ambient_payload(&raw_title);
    let body = scrub_ambient_payload(&raw_body);
    let id = format!(
        "{GENERATION_NOTIFICATION_ID_PREFIX}.{}{}",
        match input.kind {
            GenerationNotifyKind::Completed => "done",
            GenerationNotifyKind::Failed => "fail",
        },
        sanitize_id_fragment(&input.thread_id)
    );

    Some(DesktopNotificationRequest {
        id,
        title,
        body,
        default_action: Some(FOCUS_THREAD_ACTION.to_string()),
        buttons: vec![NotificationButton {
            action_id: FOCUS_THREAD_ACTION.to_string(),
            label: "Open thread".to_string(),
        }],
        thread_id: input.thread_id.clone(),
        kind: input.kind,
    })
}

/// Interpret a portal/freedesktop action id for a known thread notification.
pub fn interpret_notification_action(
    action_id: &str,
    thread_id: &str,
) -> Option<NotificationFocusAction> {
    if action_id != FOCUS_THREAD_ACTION {
        return None;
    }
    let thread_id = thread_id.trim();
    if thread_id.is_empty() {
        return None;
    }
    Some(NotificationFocusAction {
        thread_id: thread_id.to_string(),
    })
}

/// User chose to focus / open a thread from a notification action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationFocusAction {
    /// Thread to select and focus.
    pub thread_id: String,
}

fn completed_body(thread_title: Option<&str>) -> String {
    match thread_title.map(str::trim).filter(|t| !t.is_empty()) {
        Some(title) => format!("Finished in “{title}”."),
        None => "Finished generating a reply.".to_string(),
    }
}

fn failed_body(thread_title: Option<&str>, error_summary: Option<&str>) -> String {
    let base = match thread_title.map(str::trim).filter(|t| !t.is_empty()) {
        Some(title) => format!("Generation failed in “{title}”."),
        None => "Generation failed.".to_string(),
    };
    match error_summary.map(str::trim).filter(|t| !t.is_empty()) {
        Some(err) => format!("{base} {err}"),
        None => base,
    }
}

fn sanitize_id_fragment(thread_id: &str) -> String {
    let mut out = String::with_capacity(thread_id.len() + 1);
    out.push('.');
    for ch in thread_id.chars() {
        if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    out
}
