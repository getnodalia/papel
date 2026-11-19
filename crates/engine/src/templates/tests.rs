use std::sync::Arc;

use super::*;
use crate::Session;

const LETTER: (f64, f64) = (612.0, 792.0);
const A4: (f64, f64) = (595.0, 842.0);

fn bytes(id: &str, lang: Lang, page: (f64, f64)) -> Arc<Vec<u8>> {
    Session::new().create_from_template(id, lang, page).unwrap_or_else(|e| panic!("{id} ({lang:?}): {e}"))
}

/// The text a template shows, as a reader would copy it.
fn text(id: &str, lang: Lang) -> String {
    let mut session = Session::new();
    let doc = session.open_new(id, bytes(id, lang, LETTER)).unwrap();
    crate::export::Exporter::new(session.get(doc).unwrap()).text(0).unwrap()
}

#[test]
fn every_template_builds_in_both_languages_on_both_papers() {
    for id in ids() {
        for lang in [Lang::En, Lang::Es] {
            for page in [LETTER, A4] {
                let b = bytes(id, lang, page);
                let doc = papel_cos::Document::open(b.clone()).unwrap();
                assert_eq!(papel_organize::page_count(&doc).unwrap(), 1, "{id}");
                let fields = papel_forms::fields(&doc);
                assert!(fields.len() >= 4, "{id} has {} fields", fields.len());
            }
        }
    }
}

#[test]
fn only_known_ids_are_accepted() {
    let err = build("nope", Lang::En, LETTER).unwrap_err().to_string();
    assert!(err.contains("invoice") && err.contains("certificate"), "{err}");
}

#[test]
fn pages_too_small_or_not_numbers_are_refused() {
    for page in [(100.0, 100.0), (f64::NAN, 792.0), (612.0, f64::INFINITY), (-5.0, 792.0), (9e9, 9e9)] {
        assert!(build("invoice", Lang::En, page).is_err(), "{page:?}");
    }
}

#[test]
fn the_certificate_is_landscape_and_the_rest_are_portrait() {
    for id in ids() {
        let mut session = Session::new();
        let doc = session.open_new(id, bytes(id, Lang::En, LETTER)).unwrap();
        let page = &session.get(doc).unwrap().info.pages[0];
        assert_eq!(page.width > page.height, id == "certificate", "{id}: {}x{}", page.width, page.height);
    }
}

#[test]
fn language_codes_choose_spanish_only_for_spanish() {
    assert_eq!(Lang::parse("es"), Lang::Es);
    assert_eq!(Lang::parse("es-MX"), Lang::Es);
    assert_eq!(Lang::parse("ES_ar"), Lang::Es);
    assert_eq!(Lang::parse("en"), Lang::En);
    assert_eq!(Lang::parse("de"), Lang::En);
    assert_eq!(Lang::parse(""), Lang::En);
}

#[test]
fn the_gallery_lists_every_template_named_in_the_language() {
    let en = list(Lang::En);
    let es = list(Lang::Es);
    assert_eq!(en.len(), es.len());
    assert_eq!(en.len(), ids().len());
    for (a, b) in en.iter().zip(&es) {
        assert_eq!(a.id, b.id);
        assert_ne!(a.name, b.name, "{} has the same name in both languages", a.id);
        assert_ne!(a.summary, b.summary, "{} has the same summary in both languages", a.id);
    }
    for category in Category::ALL {
        assert!(en.iter().any(|t| t.category == category), "{category:?} is empty");
    }
}

/// Every word on the page must be set in the standard fonts: `win_ansi` writes `?` for what the
/// encoding can't hold, and a template must never show that.
#[test]
fn no_text_is_lost_to_the_font_encoding() {
    for id in ids() {
        for lang in [Lang::En, Lang::Es] {
            let text = text(id, lang);
            // A genuine question ends its line; a `?` inside a word is a character the font lost.
            for line in text.lines() {
                let inner = line.trim_end().trim_end_matches('?');
                assert!(!inner.contains('?'), "{id} {lang:?}: {line}");
            }
            assert!(!text.is_empty(), "{id} {lang:?} has no text");
        }
    }
}

#[test]
fn spanish_templates_use_their_accents() {
    let text = text("registration", Lang::Es);
    assert!(text.to_lowercase().contains("correo electrónico"), "{text}");
}

/// A look at the pages: `TEMPLATE_DUMP=/some/dir cargo test -p papel-engine templates::tests::dump`.
#[test]
fn dump() {
    let Some(dir) = std::env::var_os("TEMPLATE_DUMP") else { return };
    for id in ids() {
        for lang in [Lang::En, Lang::Es] {
            let b = bytes(id, lang, LETTER);
            let mut r = papel_render::PageRenderer::new(b, papel_render::RenderConfig::default());
            let out = r.render(papel_render::RenderRequest { page: 0, scale: 1.4, ..Default::default() });
            let img = image::RgbaImage::from_raw(out.width, out.height, out.rgba.to_vec()).unwrap();
            img.save(std::path::Path::new(&dir).join(format!("{id}-{}.png", lang.code()))).unwrap();
        }
    }
}
