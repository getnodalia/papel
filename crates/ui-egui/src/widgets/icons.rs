//! Vector icons: the Nodalia Design Language's bold icons, with the embedded Lucide SVGs for the
//! few NDL has no counterpart for. Both are drawn white, rasterized by egui_extras at the exact
//! on-screen pixel size, and tinted per use (Lienzo lesson: never use Unicode glyphs as icons).

use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use craft_ui::buttons::{ButtonVisuals, IconButton, IconButtonStyle};
use craft_ui::icons::{SvgIconSet, SvgStyle};

use egui::{Color32, Rect, Response, Stroke, Vec2};

use crate::icon_data::ICONS;
use crate::theme::{self, Tokens};

fn white_icons() -> &'static SvgIconSet {
    static MAP: OnceLock<SvgIconSet> = OnceLock::new();
    MAP.get_or_init(|| SvgIconSet::new(ICONS, SvgStyle { current_color: "#ffffff", stroke_width: Some("1.75") }, "bytes://icons/", "square"))
}

/// The NDL icon drawn for a name Papel uses: its own NDL name, or the NDL counterpart of a Lucide
/// name. `None` when only Lucide has it.
pub fn ndl_name(name: &str) -> Option<&'static str> {
    if let Some(icon) = nodalia_theme::icons::get(name) {
        return Some(icon.name);
    }
    let ndl = match name {
        "app-window" => "window",
        "arrow-left-right" => "arrows-horizontal",
        "arrow-up-down" => "arrows-vertical",
        "book-open" => "book",
        "check-circle-2" | "circle-check" | "badge-check" => "success",
        "circle-help" => "help",
        "circle-x" => "error",
        "clipboard-paste" => "paste",
        "clock-3" => "clock",
        "cloud-upload" => "upload",
        "code-xml" => "code",
        "columns-2" => "columns",
        "ellipsis" => "more-horizontal",
        "ellipsis-vertical" => "more-vertical",
        "file-down" => "export",
        "file-input" => "import",
        "file-plus" => "new",
        "flip-horizontal-2" => "flip",
        "flip-vertical-2" => "flip-vertical",
        "folder-open" => "open",
        "grid-3x3" => "grid",
        "hand" => "pan",
        "highlighter" => "highlight",
        "house" => "home",
        "languages" => "translate",
        "layout-grid" => "grid-view",
        "link-2" => "link",
        "list-ordered" => "list-numbers",
        "lock-open" => "unlock",
        "maximize" => "fullscreen",
        "minimize" => "fullscreen-exit",
        "message-square" | "message-square-text" => "comment",
        "message-square-plus" => "comment-add",
        "messages-square" => "comments",
        "mic" => "microphone",
        "mouse-pointer-2" => "select",
        "panel-left" => "sidebar",
        "paperclip" => "attach",
        "pen-tool" | "pen-line" => "pen",
        "pentagon" => "polygon",
        "pipette" => "eyedropper",
        "presentation" => "present",
        "printer" => "print",
        "rectangle-horizontal" => "rectangle",
        "redo-2" => "redo",
        "rotate-cw" => "rotate",
        "scissors" => "cut",
        "settings-2" => "settings",
        "share-2" => "share",
        "sparkles" => "sparkle",
        "sticky-note" => "note",
        "swatch-book" => "swatches",
        "text-cursor-input" => "text-box",
        "trash" | "trash-2" => "delete",
        "triangle-alert" => "warning",
        "type" => "text",
        "undo-2" => "undo",
        "user-round" => "user",
        "wand-sparkles" => "magic-wand",
        "x" => "close",
        "zoom-in" => "zoom",
        _ => return None,
    };
    nodalia_theme::icons::get(ndl).map(|icon| icon.name)
}

/// The SVG documents built so far, by NDL icon name.
type SvgCache = std::sync::Mutex<HashMap<&'static str, Arc<[u8]>>>;

/// An NDL icon's bold style as a white SVG document, built once per icon.
fn ndl_svg(name: &'static str) -> Option<Arc<[u8]>> {
    static CACHE: OnceLock<SvgCache> = OnceLock::new();
    let icon = nodalia_theme::icons::get(name)?;
    let mut cache = CACHE.get_or_init(Default::default).lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let svg = cache.entry(icon.name).or_insert_with(|| {
        let mut svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"24\" height=\"24\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"#ffffff\" stroke-width=\"{}\" stroke-linecap=\"round\" stroke-linejoin=\"round\">",
            nodalia_theme::icons::BOLD_STROKE
        );
        for element in icon.bold_body() {
            svg.push_str(&element.replace("currentColor", "#ffffff"));
        }
        svg.push_str("</svg>");
        Arc::from(svg.into_bytes())
    });
    Some(Arc::clone(svg))
}

pub fn exists(name: &str) -> bool {
    ndl_name(name).is_some() || white_icons().contains(name)
}

pub(crate) fn source(name: &str) -> egui::ImageSource<'static> {
    if let Some(ndl) = ndl_name(name)
        && let Some(bytes) = ndl_svg(ndl)
    {
        return egui::ImageSource::Bytes { uri: format!("bytes://ndl-icons/{ndl}.svg").into(), bytes: egui::load::Bytes::Shared(bytes) };
    }
    match white_icons().canonical_name(name) {
        Some(lucide) => white_icons().source(lucide),
        // An unknown name draws NDL's square (cached under that name).
        None => source("square"),
    }
}

pub fn image(name: &str, size: f32, tint: Color32) -> egui::Image<'static> {
    egui::Image::new(source(name)).fit_to_exact_size(Vec2::splat(size)).tint(tint)
}

pub fn paint(ui: &egui::Ui, rect: Rect, name: &str, size: f32, tint: Color32) {
    image(name, size, tint).paint_at(ui, Rect::from_center_size(rect.center(), Vec2::splat(size)));
}

/// Square icon button: transparent until hovered; `selected` gets the accent treatment.
/// `tooltip` is both its accessible name and its one tooltip.
pub fn button(ui: &mut egui::Ui, name: &str, box_size: f32, selected: bool, tooltip: &str) -> Response {
    labeled_button(ui, name, box_size, selected, tooltip, tooltip)
}

/// [`button`] whose tooltip says more than its accessible name (`label`). It shows that one
/// tooltip, so callers add no `on_hover_text` of their own (#6548: two tooltips stacked).
pub fn labeled_button(ui: &mut egui::Ui, name: &str, box_size: f32, selected: bool, label: &str, tooltip: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    // The shared button paints no fill of its own: the eased press wash below replaces it.
    let clear = |icon| ButtonVisuals { fill: Color32::TRANSPARENT, stroke: Stroke::NONE, icon };
    let style = IconButtonStyle {
        normal: clear(t.icon),
        hovered: clear(t.icon),
        pressed: Some(clear(t.icon)),
        selected: clear(t.accent_text),
        corner_radius: t.radius.into(),
        focus_stroke: Stroke::new(1.0, t.accent),
    };
    let label = if label.is_empty() { name } else { label };
    // The wash eases from the response, so reserve its place under the icon and focus ring.
    let wash = ui.painter().add(egui::Shape::Noop);
    let mut painted = None;
    let resp = IconButton::new(label, Vec2::splat(box_size), &style).selected(selected).show(ui, |_, rect, tint| painted = Some((rect, tint)));
    if let Some((rect, tint)) = painted {
        let press = theme::Press::track(ui, &resp);
        let mut shapes = Vec::with_capacity(2);
        let fill = press.wash_color(&t, selected);
        if fill.a() > 0 {
            shapes.push(egui::Shape::rect_filled(rect, t.radius, fill));
        }
        if selected {
            shapes.push(egui::Shape::rect_stroke(
                rect.shrink(0.5),
                t.radius,
                Stroke::new(1.0, t.accent.gamma_multiply(0.45)),
                egui::StrokeKind::Inside,
            ));
        }
        ui.painter().set(wash, egui::Shape::Vec(shapes));
        // The glyph shrinks a little and drops a pixel while the pointer is down.
        let size = (box_size * 0.5).round() * (1.0 - 0.08 * press.down);
        paint(ui, rect.translate(press.offset()), name, size, tint);
    }
    let resp = theme::hand(resp);
    if tooltip.is_empty() { resp } else { resp.on_hover_text(tooltip) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_catalog_icon_exists() {
        for g in papel_engine::catalog::TOOL_GROUPS {
            assert!(super::exists(g.icon), "missing icon {}", g.icon);
            for s in g.sections {
                for i in s.items {
                    assert!(super::exists(i.icon), "missing icon {}", i.icon);
                }
            }
        }
    }

    #[test]
    fn every_command_icon_exists() {
        for c in papel_engine::commands::COMMANDS {
            assert!(super::exists(c.icon), "missing icon {} ({})", c.icon, c.id);
        }
    }

    #[test]
    fn shared_catalog_preserves_existing_icons_and_fallback_uri() {
        assert_eq!(ICONS.len(), 191);
        assert!(exists("minimize-2"));
        for (name, original) in ICONS {
            if let Some(ndl) = ndl_name(name) {
                let egui::ImageSource::Bytes { uri, .. } = source(name) else { panic!("expected SVG bytes") };
                assert_eq!(uri.as_ref(), format!("bytes://ndl-icons/{ndl}.svg"));
                continue;
            }
            let expected =
                String::from_utf8_lossy(original).replace("currentColor", "#ffffff").replace("stroke-width=\"2\"", "stroke-width=\"1.75\"");
            let egui::ImageSource::Bytes { uri, bytes } = source(name) else { panic!("expected SVG bytes") };
            assert_eq!(uri.as_ref(), format!("bytes://icons/{name}.svg"));
            assert_eq!(bytes.as_ref(), expected.as_bytes(), "{name}");
        }
        let egui::ImageSource::Bytes { uri, bytes } = source("missing-icon") else { panic!("expected SVG bytes") };
        let egui::ImageSource::Bytes { bytes: fallback, .. } = source("square") else { panic!("expected SVG bytes") };
        assert_eq!(uri.as_ref(), "bytes://ndl-icons/square.svg");
        assert_eq!(bytes.as_ref(), fallback.as_ref());
        assert!(!exists("missing-icon"));
    }

    #[test]
    fn shared_button_preserves_pointer_responses() {
        // The shared button must respond like the original allocate-and-paint path in both
        // themes. Its painting differs on purpose (the eased press wash, the selected outline
        // and the sinking glyph); the keyboard-focus outline is covered in craft-ui.
        fn frame(
            ctx: &egui::Context,
            events: Vec<egui::Event>,
            selected: bool,
            enabled: bool,
            legacy: bool,
        ) -> (egui::Response, Vec<egui::epaint::ClippedShape>) {
            let mut response = None;
            let mut output = ctx.run_ui(egui::RawInput { events, ..Default::default() }, |ui| {
                response = Some(
                    ui.add_enabled_ui(enabled, |ui| {
                        if legacy {
                            let t = Tokens::get(ui.ctx());
                            let (rect, response) = ui.allocate_exact_size(Vec2::splat(28.0), egui::Sense::click());
                            if selected {
                                ui.painter().rect_filled(rect, t.radius, t.accent_soft);
                            } else if response.is_pointer_button_down_on() {
                                ui.painter().rect_filled(rect, t.radius, t.pressed);
                            } else if response.hovered() {
                                ui.painter().rect_filled(rect, t.radius, t.hover);
                            }
                            let tint = if selected { t.accent_text } else { t.icon };
                            paint(ui, rect, "square", (28.0_f32 * 0.5).round(), tint);
                            response
                        } else {
                            button(ui, "square", 28.0, selected, "")
                        }
                    })
                    .inner,
                );
            });
            output.textures_delta.clear();
            (response.unwrap(), output.shapes)
        }
        for theme in [crate::theme::ThemeKind::Dark, crate::theme::ThemeKind::Light] {
            for selected in [false, true] {
                for enabled in [false, true] {
                    let old = egui::Context::default();
                    let new = egui::Context::default();
                    for ctx in [&old, &new] {
                        crate::theme::apply(ctx, theme);
                        egui_extras::install_image_loaders(ctx);
                    }
                    let mut center = egui::Pos2::ZERO;
                    for step in 0..7 {
                        let events = match step {
                            2 => vec![egui::Event::PointerMoved(center)],
                            3 | 4 => vec![egui::Event::PointerButton {
                                pos: center,
                                button: egui::PointerButton::Primary,
                                pressed: step == 3,
                                modifiers: egui::Modifiers::NONE,
                            }],
                            5 => vec![egui::Event::PointerMoved(egui::pos2(300.0, 200.0))],
                            _ => vec![],
                        };
                        let (a, _) = frame(&old, events.clone(), selected, enabled, true);
                        let (b, after) = frame(&new, events, selected, enabled, false);
                        center = a.rect.center();
                        assert_eq!(a.rect, b.rect);
                        assert_eq!(
                            (a.clicked(), a.hovered(), a.is_pointer_button_down_on()),
                            (b.clicked(), b.hovered(), b.is_pointer_button_down_on())
                        );
                        assert!(!after.is_empty(), "{theme:?}, selected={selected}, enabled={enabled}, step={step}");
                    }
                }
            }
        }
    }
}
