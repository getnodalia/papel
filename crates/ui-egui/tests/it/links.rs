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
