//! Who draws the window frame on Linux and the BSDs.
//!
//! Papel draws the Nodalia Design Language's window there: winit's frame is off, the tab strip is
//! the header bar and Nodalia's round window buttons sit at its end
//! (`papel_ui_egui::window_controls`). Preferences › Use system title bar gives the desktop its
//! own frame back. macOS and Windows always keep the system's (the caller never asks here).
//!
//! `PAPEL_TITLEBAR=system` starts with the desktop's frame whatever the preference says.

use papel_ui_egui::window_controls::Layout;

/// The window buttons to draw in the app, or `None` when the desktop draws the frame.
pub(crate) fn app_drawn() -> Option<Layout> {
    let system = std::env::var("PAPEL_TITLEBAR").ok().is_some_and(|v| v.trim() == "system");
    (!system).then(Layout::default)
}
