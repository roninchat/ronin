//! XDG Desktop Portal notification delivery for generation done/fail.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

use ashpd::desktop::notification::{Button, Notification, NotificationProxy, Priority};
use futures_util::StreamExt;
use ronin_core::{
    DesktopNotificationRequest, DesktopNotifier, NotificationError, FOCUS_THREAD_ACTION,
};

/// Delivers shaped requests via `org.freedesktop.portal.Notification`.
///
/// Also listens for portal `ActionInvoked` signals so a focus/open-thread click
/// can be drained via [`DesktopNotifier::poll_focus_thread`].
#[derive(Debug, Clone)]
pub struct PortalDesktopNotifier {
    state: Arc<PortalNotifierState>,
}

#[derive(Debug)]
struct PortalNotifierState {
    id_to_thread: Mutex<HashMap<String, String>>,
    focus_tx: Sender<String>,
    focus_rx: Mutex<Receiver<String>>,
    listener_started: AtomicBool,
}

impl Default for PortalDesktopNotifier {
    fn default() -> Self {
        Self::new()
    }
}

impl PortalDesktopNotifier {
    /// Creates a portal notifier with an idle focus-action queue.
    pub fn new() -> Self {
        let (focus_tx, focus_rx) = mpsc::channel();
        Self {
            state: Arc::new(PortalNotifierState {
                id_to_thread: Mutex::new(HashMap::new()),
                focus_tx,
                focus_rx: Mutex::new(focus_rx),
                listener_started: AtomicBool::new(false),
            }),
        }
    }

    fn ensure_action_listener(&self) {
        if self.state.listener_started.swap(true, Ordering::SeqCst) {
            return;
        }
        let state = Arc::clone(&self.state);
        std::thread::Builder::new()
            .name("ronin-notification-actions".into())
            .spawn(move || {
                if let Err(e) = listen_for_actions(state) {
                    tracing::warn!(%e, "desktop notification action listener stopped");
                }
            })
            .ok();
    }
}

impl DesktopNotifier for PortalDesktopNotifier {
    fn notify(&self, request: &DesktopNotificationRequest) -> Result<(), NotificationError> {
        self.ensure_action_listener();
        {
            let mut map = self
                .state
                .id_to_thread
                .lock()
                .unwrap_or_else(|p| p.into_inner());
            map.insert(request.id.clone(), request.thread_id.clone());
        }
        deliver_via_portal(request)
    }

    fn poll_focus_thread(&self) -> Option<String> {
        let rx = self
            .state
            .focus_rx
            .lock()
            .unwrap_or_else(|p| p.into_inner());
        rx.try_recv().ok()
    }
}

fn listen_for_actions(state: Arc<PortalNotifierState>) -> Result<(), String> {
    async_std::task::block_on(async {
        let proxy = NotificationProxy::new().await.map_err(|e| e.to_string())?;
        let mut actions = proxy
            .receive_action_invoked()
            .await
            .map_err(|e| e.to_string())?;
        while let Some(action) = actions.next().await {
            if action.name() != FOCUS_THREAD_ACTION {
                continue;
            }
            let thread_id = {
                let map = state.id_to_thread.lock().unwrap_or_else(|p| p.into_inner());
                map.get(action.id()).cloned()
            };
            if let Some(thread_id) = thread_id {
                let _ = state.focus_tx.send(thread_id);
            }
        }
        Ok(())
    })
}

fn deliver_via_portal(request: &DesktopNotificationRequest) -> Result<(), NotificationError> {
    async_std::task::block_on(async {
        let proxy = NotificationProxy::new()
            .await
            .map_err(|e| NotificationError::DeliveryFailed(e.to_string()))?;

        let mut notification = Notification::new(request.title.as_str())
            .body(request.body.as_str())
            .priority(Priority::Normal);

        if let Some(action) = request.default_action.as_deref() {
            notification = notification.default_action(action);
        }

        for button in &request.buttons {
            notification = notification.button(Button::new(
                button.label.as_str(),
                button.action_id.as_str(),
            ));
        }

        proxy
            .add_notification(request.id.as_str(), notification)
            .await
            .map_err(|e| NotificationError::DeliveryFailed(e.to_string()))?;
        Ok(())
    })
}

/// Maps a portal action invocation into a focus-thread id when applicable.
pub fn focus_thread_id_from_action(
    action_name: &str,
    request: &DesktopNotificationRequest,
) -> Option<String> {
    ronin_core::interpret_notification_action(action_name, &request.thread_id).map(|a| a.thread_id)
}
