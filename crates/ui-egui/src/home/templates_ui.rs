//! Home's templates: a wrapping grid of page previews, grouped, that start a new document.
//! The words around them come with the templates (English and Spanish; other languages read
//! English), so the gallery and its pages always speak the same language.

use egui::{Align2, CornerRadius, Rect, Sense, Stroke, TextureOptions, vec2};
use papel_engine::templates::{self, Lang, TemplateInfo, Words};

use crate::theme::{self, Tokens};
use crate::{PapelApp, widgets};

/// The narrowest and widest a tile gets: tiles grow to share a row's width, within these.
const TILE_MIN: f32 = 150.0;
const TILE_MAX: f32 = 196.0;
const GAP: f32 = 16.0;
/// Previews drawn per frame, so the first visit to Home doesn't stall on thirteen pages.
const PREVIEWS_PER_FRAME: usize = 2;

/// How many tiles of at least [`TILE_MIN`] fit across `width`, and how wide each one is.
pub(crate) fn tile_grid(width: f32) -> (usize, f32) {
    let per_row = (((width + GAP) / (TILE_MIN + GAP)).floor() as usize).max(1);
    let tile = ((width - GAP * (per_row as f32 - 1.0)) / per_row as f32).clamp(TILE_MIN.min(width.max(40.0)), TILE_MAX);
    (per_row, tile)
}

/// The language the templates are written in: the interface language, when Papel has the
/// templates in it.
pub(crate) fn language() -> Lang {
    Lang::parse(crate::i18n::current().code())
}

fn key(id: &str, lang: Lang, page: (f64, f64)) -> String {
    format!("{id}|{}|{:.0}x{:.0}", lang.code(), page.0, page.1)
}

/// The preview of one template, rendered on first use and kept. `None` while it waits its turn.
fn preview(app: &mut PapelApp, ctx: &egui::Context, id: &str, lang: Lang, budget: &mut usize) -> Option<egui::TextureHandle> {
    let page = app.blank_paper.points();
    let key = key(id, lang, page);
    if let Some(texture) = app.template_previews.get(&key) {
        return Some(texture.clone());
    }
    if *budget == 0 {
        return None;
    }
    *budget -= 1;
    let bytes = app.session.create_from_template(id, lang, page).ok()?;
    let config = papel_render::RenderConfig::default().on_screen();
    let mut renderer = papel_render::PageRenderer::new(bytes, config);
    let scale = if id == "certificate" { 0.5 } else { 0.42 };
    let out = renderer.render(papel_render::RenderRequest { page: 0, scale, ..Default::default() });
    if out.error.is_some() || out.width == 0 {
        return None;
    }
    let image = egui::ColorImage::from_rgba_premultiplied([out.width as usize, out.height as usize], &out.rgba);
    let texture = ctx.load_texture(format!("template-{key}"), image, TextureOptions::LINEAR);
    app.template_previews.insert(key, texture.clone());
    Some(texture)
}

/// One tile: the page, its name, and one line about it. Returns true when it was clicked.
fn tile(ui: &mut egui::Ui, t: &Tokens, info: &TemplateInfo, texture: Option<&egui::TextureHandle>, width: f32, landscape: bool) -> bool {
    // Every tile has the same height, so a row stays level; a landscape page sits in the middle of its mat.
    let thumb_h = width * 1.2;
    let text_h = 66.0;
    let (rect, resp) = ui.allocate_exact_size(vec2(width, thumb_h + text_h + 16.0), Sense::click());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, info.name));
    let press = theme::Press::track(ui, &resp);
    super::card_lift(ui, t, rect, press);
    ui.painter().rect(rect, CornerRadius::same(10), press.fill(t.card, t.hover, t.pressed), Stroke::new(1.0, t.divider), egui::StrokeKind::Inside);
    let body = rect.translate(press.offset());

    // The page, centred on a quiet mat.
    let mat = Rect::from_min_size(body.min + vec2(8.0, 8.0), vec2(width - 16.0, thumb_h));
    ui.painter().rect_filled(mat, CornerRadius::same(6), t.pasteboard);
    if let Some(texture) = texture {
        let size = texture.size_vec2();
        let fit = (mat.width() / size.x).min(mat.height() / size.y);
        let page = Rect::from_center_size(mat.center(), size * fit * if landscape { 1.0 } else { 0.9 });
        ui.painter().rect_filled(page.translate(vec2(0.0, 2.0)).expand(0.5), CornerRadius::same(2), t.page_shadow.gamma_multiply(0.5));
        ui.painter().image(texture.id(), page, Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), egui::Color32::WHITE);
    } else {
        ui.painter().text(mat.center(), Align2::CENTER_CENTER, "…", theme::regular(16.0), t.text_faint);
    }

    let title = body.min + vec2(12.0, thumb_h + 22.0);
    widgets::row_text(ui, title, info.name, theme::semibold(13.0), t.text, width - 24.0);
    let mut job = egui::text::LayoutJob::simple(info.summary.to_owned(), theme::regular(11.0), t.text_muted, width - 24.0);
    job.wrap.max_rows = 2;
    let galley = ui.fonts_mut(|f| f.layout_job(job));
    ui.painter().galley(body.min + vec2(12.0, thumb_h + 34.0), galley, t.text_muted);
    theme::hand(resp).clicked()
}

/// The Home section: heading, a line about it, then every template, a group at a time.
pub(crate) fn section(app: &mut PapelApp, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    let lang = language();
    let words: Words = templates::words(lang);
    ui.label(egui::RichText::new(words.heading).font(theme::semibold(17.0)));
    ui.add_space(2.0);
    ui.label(egui::RichText::new(words.intro).color(t.text_muted).font(theme::regular(12.5)));

    let list = templates::list(lang);
    let ctx = ui.ctx().clone();
    let mut budget = PREVIEWS_PER_FRAME;
    let mut pending = false;
    let mut chosen = None;
    let (per_row, width) = tile_grid(ui.available_width());
    for category in templates::Category::ALL {
        let group: Vec<&TemplateInfo> = list.iter().filter(|i| i.category == category).collect();
        if group.is_empty() {
            continue;
        }
        ui.add_space(14.0);
        ui.label(egui::RichText::new(category.label(lang)).color(t.text_muted).font(theme::medium(12.0)));
        ui.add_space(6.0);
        for row in group.chunks(per_row) {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = vec2(GAP, GAP);
                for info in row {
                    let texture = preview(app, &ctx, info.id, lang, &mut budget);
                    pending |= texture.is_none();
                    if tile(ui, &t, info, texture.as_ref(), width, info.id == "certificate") {
                        chosen = Some(info.id);
                    }
                }
            });
            ui.add_space(GAP - 4.0);
        }
    }
    if pending {
        ctx.request_repaint();
    }
    if let Some(id) = chosen {
        app.create_from_template(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiles_wrap_instead_of_running_off_the_edge() {
        for width in [120.0, 300.0, 520.0, 700.0, 1000.0, 1400.0, 2400.0] {
            let (per_row, tile) = tile_grid(width);
            assert!(per_row >= 1);
            let used = per_row as f32 * tile + (per_row as f32 - 1.0) * GAP;
            assert!(used <= width + 0.5 || per_row == 1, "{per_row} tiles of {tile} overflow {width}");
            assert!(tile <= TILE_MAX);
        }
        assert_eq!(tile_grid(1400.0).0, 8);
        assert_eq!(tile_grid(300.0).0, 1);
    }
}
