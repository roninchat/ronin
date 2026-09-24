//! In-theme companion sprite and a tiled grain overlay.

use std::time::Duration;

use gpui::{
    canvas, div, point, px, size, Bounds, Hsla, IntoElement, ParentElement, SharedString, Styled,
    TransformationMatrix,
};

/// Pose shown by the in-theme companion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompanionMood {
    /// Resting pose, with an occasional blink.
    Idle,
    /// Alert pose used when a new chat starts.
    NewChat,
    /// Happy pose held after a message is sent.
    Sent,
}

/// How long a reaction frame stays up before returning to idle.
pub fn reaction_hold() -> Duration {
    Duration::from_millis(1400)
}

/// Asset path for `mood` at `elapsed_ms` since the companion started animating.
pub fn companion_sprite(mood: CompanionMood, elapsed_ms: u64) -> &'static str {
    match mood {
        CompanionMood::Sent => "companion/sent.svg",
        CompanionMood::NewChat => "companion/new-chat.svg",
        CompanionMood::Idle => {
            if (elapsed_ms / 2800) % 6 == 5 {
                "companion/idle-1.svg"
            } else {
                "companion/idle-0.svg"
            }
        }
    }
}

/// Decorative companion sprite. It does not capture pointer events.
pub fn render_companion(mood: CompanionMood, elapsed_ms: u64, color: Hsla) -> impl IntoElement {
    let path = companion_sprite(mood, elapsed_ms);
    let bob = ((elapsed_ms as f32 / 700.0).sin() + 1.0) * 1.0;
    div()
        .w(px(36.))
        .h(px(36.))
        .flex()
        .items_center()
        .justify_center()
        .child(
            gpui::svg()
                .path(path)
                .text_color(color)
                .size(px(32.))
                .mt(px(bob)),
        )
}

/// Full-size tiled grain mask tinted with `tint`.
pub fn grain_overlay(tint: Hsla) -> impl IntoElement {
    let tint = tint;
    canvas(
        |_bounds, _window, _cx| (),
        move |bounds, _state, window, cx| {
            // One sprite per 128px keeps the speckle and cuts paint calls by about 4x.
            let tile = px(128.0);
            let tile_f: f32 = tile.into();
            let origin_x: f32 = bounds.origin.x.into();
            let origin_y: f32 = bounds.origin.y.into();
            let width: f32 = bounds.size.width.into();
            let height: f32 = bounds.size.height.into();
            let mut y = origin_y;
            while y < origin_y + height {
                let mut x = origin_x;
                while x < origin_x + width {
                    let b = Bounds {
                        origin: point(px(x), px(y)),
                        size: size(tile, tile),
                    };
                    let _ = window.paint_svg(
                        b,
                        SharedString::from("textures/grain.svg"),
                        TransformationMatrix::default(),
                        tint,
                        cx,
                    );
                    x += tile_f;
                }
                y += tile_f;
            }
        },
    )
    .absolute()
    .inset_0()
    .size_full()
    .opacity(0.28)
}
