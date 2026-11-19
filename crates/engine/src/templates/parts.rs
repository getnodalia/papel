//! Pieces several templates share: the title, a labelled line to fill in, a section heading.

use super::sheet::{Align, Entry, Face, INK, MUTED, RULE, Sheet};

impl Sheet {
    /// The template's title: a short accent bar over large type. Returns the baseline.
    pub(crate) fn title(&mut self, x: f64, y: f64, s: &str) -> f64 {
        let accent = self.accent;
        self.rect(x, y, 30.0, 4.0, accent);
        self.text(x, y + 38.0, Align::Left, Face::SansBold, 28.0, INK, 0.0, s);
        y + 38.0
    }

    /// A small label over a rule, with a text field between them. `y` is the top of the label;
    /// the whole thing is 32 points tall.
    pub(crate) fn lined(&mut self, name: &str, label: &str, x: f64, y: f64, w: f64) {
        self.lined_as(name, label, x, y, w, Entry::Text);
    }

    pub(crate) fn lined_as(&mut self, name: &str, label: &str, x: f64, y: f64, w: f64, entry: Entry) {
        self.label(x, y + 8.0, Align::Left, MUTED, label);
        self.rule(x, x + w, y + 31.0, 0.6, RULE);
        self.field(name, x, y + 12.0, w, 18.0, entry);
    }

    /// A section heading: the label in the accent colour, with a rule under it.
    pub(crate) fn section(&mut self, label: &str, x: f64, y: f64, w: f64) {
        let accent = self.accent;
        self.label(x, y, Align::Left, accent, label);
        self.rule(x, x + w, y + 7.0, 0.6, RULE);
    }

    /// A soft panel with a multi-line field in it. `y` is the panel's top.
    pub(crate) fn writing_panel(&mut self, name: &str, x: f64, y: f64, w: f64, h: f64) {
        self.panel(x, y, w, h, 6.0, super::sheet::WASH);
        self.field(name, x + 8.0, y + 6.0, w - 16.0, h - 12.0, Entry::Multiline);
    }

    /// A tick box with its caption to the right.
    pub(crate) fn tick(&mut self, name: &str, caption: &str, x: f64, y: f64) {
        self.field(name, x, y, 12.0, 12.0, Entry::Check);
        self.text(x + 20.0, y + 9.5, Align::Left, Face::Sans, 9.5, INK, 0.0, caption);
    }
}
