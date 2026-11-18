//! Project links: the main menu is one click away in the top bar; the Help menu and the About
//! dialog open the Papel and Nodalia pages and the issue tracker.

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use papel_engine::links;
use papel_ui_egui::{Dialog, PapelApp};

fn harness(setup: impl FnOnce(&mut PapelApp) + 'static) -> Harness<'static, PapelApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(move |_cc| {
        let mut app = PapelApp::new();
        app.set_option("language", "en").unwrap();
        setup(&mut app);
        app
    });
    h.run_steps(4);
    h
}

#[test]
fn main_menu_opens_about() {
    let mut h = harness(|_| {});
    h.get_by_label("Main menu").click();
    h.run_steps(2);
    h.get_by_label("About Papel").click();
    h.run_steps(2);
    assert_eq!(h.state().dialog, Some(Dialog::About));
}

#[test]
fn about_dialog_shows_the_app_and_links() {
    let mut h = harness(|app| app.dialog = Some(Dialog::About));
    h.run_steps(4);
    h.get_by_label("Part of Nodalia");
    h.get_by_label("Report a problem").click();
    h.run_steps(2);
    assert_eq!(h.state().last_opened_url.as_deref(), Some(links::ISSUES));
    assert_eq!(links::ISSUES, "https://github.com/getnodalia/papel/issues");
}

#[test]
fn help_commands_open_each_link() {
    for l in links::LINKS {
        let mut h = harness(|_| {});
        assert!(h.state_mut().execute(l.command), "{}", l.command);
        assert_eq!(h.state().last_opened_url.as_deref(), Some(l.url));
        assert_eq!(papel_engine::commands::command(l.command).unwrap().menu, Some("Help"));
    }
}

/// Runs frames until the node labelled `label` exists and has stopped moving, then returns it.
///
/// The About dialog is a modal centred on its own size, and switching tabs changes that size, so
/// for a few frames afterwards the modal grows and recentres and every control in it moves.
fn settled<'a>(h: &'a mut Harness<'static, PapelApp>, label: &'a str) -> egui_kittest::Node<'a> {
    let mut last = None;
    for _ in 0..60 {
        h.run_steps(1);
        let now = h.query_by_label(label).map(|n| n.rect());
        if now.is_some() && now == last {
            return h.get_by_label(label);
        }
        last = now;
    }
    panic!("{label:?} never appeared or never stopped moving");
}

#[test]
fn about_dialog_has_contributors_and_models_tabs() {
    let mut h = harness(|app| app.dialog = Some(Dialog::About));
    settled(&mut h, "Contributors").click();
    settled(&mut h, "@echelon");
    settled(&mut h, "Table").click();
    settled(&mut h, "PRs");
    settled(&mut h, "Display name").click();
    settled(&mut h, "Brandon Thomas");
    settled(&mut h, "Models").click();
    h.run_steps(4);
    assert!(h.query_all_by_label("Anthropic").count() >= 1);
}
