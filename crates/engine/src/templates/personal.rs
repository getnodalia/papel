//! Personal pages: résumé, weekly planner, checklist, study notes, certificate.

use super::Lang;
use super::sheet::{Align, Entry, Face, INK, MUTED, RULE, Sheet, WASH};

pub(super) fn resume(s: &mut Sheet, lang: Lang) {
    let (m, w, h) = (s.margin(), s.w, s.h);
    let accent = s.accent;

    // A soft column down the left edge for the facts; the story goes beside it.
    let side = (w * 0.32).round();
    s.rect(0.0, 0.0, side, h, accent.tint(0.9));
    let sx = 28.0;
    let sw = side - 2.0 * sx;
    s.rect(sx, m, 30.0, 4.0, accent);
    let blocks = [
        ("Contact", lang.pick("Contact", "Contacto"), 0.20),
        ("Skills", lang.pick("Skills", "Habilidades"), 0.34),
        ("Languages", lang.pick("Languages", "Idiomas"), 0.20),
        ("Links", lang.pick("Links", "Enlaces"), 0.14),
    ];
    let total = h - 2.0 * m - 40.0;
    let mut y = m + 36.0;
    for (name, label, share) in blocks {
        let bh = total * share;
        s.label(sx, y, Align::Left, accent, label);
        s.rule(sx, sx + sw, y + 7.0, 0.6, accent.tint(0.5));
        s.field(name, sx, y + 14.0, sw, bh - 30.0, Entry::Multiline);
        y += bh;
    }

    // The main column.
    let mx = side + 36.0;
    let mw = w - mx - m;
    s.field_sized("Name", mx, m, mw, 36.0, Entry::Text, 26.0);
    s.field_sized("Headline", mx, m + 40.0, mw, 22.0, Entry::Text, 12.0);
    s.rule(mx, mx + mw, m + 74.0, 1.0, accent);

    let top = m + 98.0;
    let avail = h - m - top;
    let parts = [
        ("Profile", lang.pick("Profile", "Perfil"), 0.2),
        ("Experience", lang.pick("Experience", "Experiencia"), 0.5),
        ("Education", lang.pick("Education", "Formación"), 0.3),
    ];
    let mut y = top;
    for (name, label, share) in parts {
        let bh = avail * share;
        s.label(mx, y, Align::Left, accent, label);
        s.rule(mx, mx + mw, y + 7.0, 0.6, RULE);
        s.field(name, mx, y + 14.0, mw, bh - 30.0, Entry::Multiline);
        y += bh;
    }
}

pub(super) fn planner(s: &mut Sheet, lang: Lang) {
    let (m, w, h) = (s.margin(), s.w, s.h);
    let cw = w - 2.0 * m;
    let accent = s.accent;

    s.title(m, m, lang.pick("Weekly planner", "Planificador semanal"));
    s.lined_as("Week of", lang.pick("Week starting", "Semana del"), w - m - 168.0, m - 6.0, 168.0, Entry::Date);
    s.lined("Focus", lang.pick("This week I will focus on", "Esta semana me centraré en"), m, m + 62.0, cw);

    let top = m + 112.0;
    let gap = 14.0;
    let cell_w = (cw - gap) / 2.0;
    let cell_h = (h - m - top - 3.0 * gap) / 4.0;
    let cells = [
        ("Monday", lang.pick("Monday", "Lunes")),
        ("Tuesday", lang.pick("Tuesday", "Martes")),
        ("Wednesday", lang.pick("Wednesday", "Miércoles")),
        ("Thursday", lang.pick("Thursday", "Jueves")),
        ("Friday", lang.pick("Friday", "Viernes")),
        ("Saturday", lang.pick("Saturday", "Sábado")),
        ("Sunday", lang.pick("Sunday", "Domingo")),
        ("Notes", lang.pick("Notes", "Notas")),
    ];
    for (i, (name, label)) in cells.into_iter().enumerate() {
        let (col, row) = (i % 2, i / 2);
        let x = m + col as f64 * (cell_w + gap);
        let y = top + row as f64 * (cell_h + gap);
        if i >= 5 {
            s.panel(x, y, cell_w, cell_h, 8.0, WASH);
        } else {
            s.outline(x, y, cell_w, cell_h, 8.0, 0.8, RULE);
        }
        s.label(x + 14.0, y + 20.0, Align::Left, if i == 7 { MUTED } else { accent }, label);
        s.field(name, x + 10.0, y + 28.0, cell_w - 20.0, cell_h - 36.0, Entry::Multiline);
    }
}

pub(super) fn checklist(s: &mut Sheet, lang: Lang) {
    let (m, w, h) = (s.margin(), s.w, s.h);
    let cw = w - 2.0 * m;
    let accent = s.accent;

    s.title(m, m, lang.pick("Checklist", "Lista de tareas"));
    s.lined_as("Date", lang.pick("Date", "Fecha"), w - m - 150.0, m - 6.0, 150.0, Entry::Date);

    let top = m + 70.0;
    s.section(lang.pick("To do", "Pendiente"), m, top, cw);
    let notes_h = 112.0;
    let rows = (((h - m - notes_h - 30.0) - (top + 16.0)) / 29.0).floor().max(6.0) as usize;
    for i in 0..rows {
        let y = top + 16.0 + i as f64 * 29.0;
        s.rule(m, m + cw, y + 28.0, 0.5, RULE);
        let n = i + 1;
        s.field(&format!("{} {n}", lang.pick("Done", "Hecho")), m + 2.0, y + 8.0, 13.0, 13.0, Entry::Check);
        s.field(&format!("{} {n}", lang.pick("Task", "Tarea")), m + 30.0, y + 5.0, cw - 30.0, 19.0, Entry::Text);
    }
    let y = top + 16.0 + rows as f64 * 29.0 + 24.0;
    s.label(m, y, Align::Left, accent, lang.pick("Notes", "Notas"));
    s.writing_panel("Notes", m, y + 10.0, cw, (h - m - y - 10.0).max(60.0));
}

pub(super) fn notes(s: &mut Sheet, lang: Lang) {
    let (m, w, h) = (s.margin(), s.w, s.h);
    let cw = w - 2.0 * m;
    let accent = s.accent;

    s.title(m, m, lang.pick("Study notes", "Apuntes de estudio"));
    let y = m + 58.0;
    s.lined("Topic", lang.pick("Topic", "Tema"), m, y, cw * 0.62);
    s.lined_as("Date", lang.pick("Date", "Fecha"), m + cw * 0.70, y, cw * 0.30, Entry::Date);

    // Cues beside the notes, a summary under both.
    let top = y + 52.0;
    let summary_h = 120.0;
    let bottom = h - m - summary_h - 28.0;
    let cue_w = cw * 0.28;
    s.label(m, top, Align::Left, accent, lang.pick("Cues", "Palabras clave"));
    s.label(m + cue_w + 22.0, top, Align::Left, accent, lang.pick("Notes", "Apuntes"));
    s.panel(m, top + 10.0, cue_w, bottom - top - 10.0, 6.0, WASH);
    s.field("Cues", m + 8.0, top + 16.0, cue_w - 16.0, bottom - top - 22.0, Entry::Multiline);
    s.line(m + cue_w + 11.0, top + 10.0, m + cue_w + 11.0, bottom, 0.8, RULE);
    s.field("Notes", m + cue_w + 22.0, top + 16.0, cw - cue_w - 22.0, bottom - top - 22.0, Entry::Multiline);
    s.rule(m, m + cw, bottom + 14.0, 0.8, accent);
    s.label(m, bottom + 32.0, Align::Left, accent, lang.pick("Summary", "Resumen"));
    s.writing_panel("Summary", m, bottom + 42.0, cw, summary_h - 14.0);
}

pub(super) fn certificate(s: &mut Sheet, lang: Lang) {
    let (w, h) = (s.w, s.h);
    let accent = s.accent;
    let mid = w / 2.0;

    // A double frame.
    s.outline(24.0, 24.0, w - 48.0, h - 48.0, 4.0, 1.6, accent);
    s.outline(34.0, 34.0, w - 68.0, h - 68.0, 2.0, 0.5, accent);

    let title_y = h * 0.26;
    s.text(mid, title_y, Align::Center, Face::SerifBold, 46.0, INK, 0.5, lang.pick("Certificate", "Certificado"));
    s.text(mid, title_y + 26.0, Align::Center, Face::SansBold, 9.0, accent, 3.0, lang.pick("OF ACHIEVEMENT", "DE LOGRO"));

    // An ornament: a diamond between two short rules.
    let orn = title_y + 52.0;
    s.rule(mid - 90.0, mid - 12.0, orn, 0.8, accent);
    s.rule(mid + 12.0, mid + 90.0, orn, 0.8, accent);
    s.diamond(mid, orn, 4.0, accent);

    s.text(
        mid,
        h * 0.46,
        Align::Center,
        Face::SerifItalic,
        14.0,
        MUTED,
        0.0,
        lang.pick("This certificate is proudly presented to", "Este certificado se entrega con orgullo a"),
    );
    let name_y = h * 0.48;
    s.field_serif("Recipient", w * 0.2, name_y, w * 0.6, 44.0, Entry::Centered, 30.0);
    s.rule(w * 0.24, w * 0.76, name_y + 48.0, 0.8, MUTED);

    s.text(mid, h * 0.62, Align::Center, Face::SerifItalic, 14.0, MUTED, 0.0, lang.pick("in recognition of", "en reconocimiento a"));
    s.field_serif("Achievement", w * 0.2, h * 0.64, w * 0.6, 44.0, Entry::CenteredMulti, 14.0);

    // Date and signature, apart.
    let y = h - 112.0;
    for (i, (name, label)) in [("Date", lang.pick("Date", "Fecha")), ("Signature", lang.pick("Signature", "Firma"))].into_iter().enumerate() {
        let x = if i == 0 { w * 0.16 } else { w * 0.58 };
        let width = w * 0.26;
        s.field_serif(name, x, y - 24.0, width, 22.0, Entry::Centered, 13.0);
        s.rule(x, x + width, y, 0.8, MUTED);
        s.label(x + width / 2.0, y + 16.0, Align::Center, MUTED, label);
    }
}
