//! Forms to fill in: registration, timesheet, expense report.

use super::Lang;
use super::sheet::{Align, Entry, Face, INK, MUTED, RULE, Rgb, Sheet};

pub(super) fn registration(s: &mut Sheet, lang: Lang) {
    let (m, w) = (s.margin(), s.w);
    let cw = w - 2.0 * m;
    let extra = (s.h - 792.0).max(0.0);

    s.title(m, m, lang.pick("Registration form", "Formulario de inscripción"));
    let intro = lang.pick(
        "Please fill in the form and sign at the bottom. Fields marked with a line are typed straight into the PDF.",
        "Rellena el formulario y firma al final. Los campos con línea se escriben directamente en el PDF.",
    );
    s.paragraph(m, m + 62.0, cw * 0.8, Face::Sans, 10.0, 14.0, MUTED, intro);

    let half = (cw - 24.0) / 2.0;
    let y = m + 102.0;
    s.section(lang.pick("About you", "Sobre ti"), m, y, cw);
    s.lined("Full name", lang.pick("Full name", "Nombre completo"), m, y + 20.0, cw);
    s.lined("Email", lang.pick("Email", "Correo electrónico"), m, y + 62.0, half);
    s.lined("Phone", lang.pick("Phone", "Teléfono"), m + half + 24.0, y + 62.0, half);
    s.lined_as("Birth date", lang.pick("Date of birth", "Fecha de nacimiento"), m, y + 104.0, half, Entry::Date);
    s.lined("Organisation", lang.pick("Organisation", "Organización"), m + half + 24.0, y + 104.0, half);

    let y = y + 164.0;
    s.section(lang.pick("Your choices", "Tus opciones"), m, y, cw);
    let how = vec![
        lang.pick("Search engine", "Buscador").to_string(),
        lang.pick("A friend", "Un conocido").to_string(),
        lang.pick("Social media", "Redes sociales").to_string(),
        lang.pick("An event", "Un evento").to_string(),
        lang.pick("Other", "Otro").to_string(),
    ];
    s.lined_as("Heard about us", lang.pick("How did you hear about us?", "¿Cómo nos conociste?"), m, y + 20.0, half, Entry::Choice(how));
    let sessions =
        vec![lang.pick("Morning", "Mañana").to_string(), lang.pick("Afternoon", "Tarde").to_string(), lang.pick("Evening", "Noche").to_string()];
    s.lined_as("Preferred session", lang.pick("Preferred session", "Turno preferido"), m + half + 24.0, y + 20.0, half, Entry::Choice(sessions));
    s.tick("Updates", lang.pick("Send me news and updates", "Quiero recibir novedades"), m, y + 74.0);
    s.tick("Terms", lang.pick("I have read and accept the terms", "He leído y acepto las condiciones"), m, y + 98.0);

    let y = y + 140.0;
    s.section(lang.pick("Anything else?", "¿Algo más?"), m, y, cw);
    s.writing_panel("Comments", m, y + 14.0, cw, 90.0 + extra * 0.5);

    let y = s.h - m - 56.0;
    s.rule(m, m + cw * 0.55, y + 30.0, 0.6, MUTED);
    s.label(m, y + 44.0, Align::Left, MUTED, lang.pick("Signature", "Firma"));
    s.lined_as("Signature date", lang.pick("Date", "Fecha"), m + cw * 0.66, y + 2.0, cw * 0.34, Entry::Date);
}

/// A table of ruled rows under a row of column labels; returns the top of the first row.
fn header(s: &mut Sheet, y: f64, cols: &[(f64, f64, Align, &str)]) -> f64 {
    let accent = s.accent;
    let (m, cw) = (s.margin(), s.w - 2.0 * s.margin());
    s.panel(m, y, cw, 24.0, 5.0, accent);
    for (x, w, align, label) in cols {
        let at = match align {
            Align::Right => x + w - 8.0,
            Align::Center => x + w / 2.0,
            Align::Left => x + 8.0,
        };
        s.label(at, y + 15.5, *align, Rgb(1.0, 1.0, 1.0), label);
    }
    y + 24.0
}

pub(super) fn timesheet(s: &mut Sheet, lang: Lang) {
    let (m, w) = (s.margin(), s.w);
    let cw = w - 2.0 * m;
    let accent = s.accent;

    s.title(m, m, lang.pick("Timesheet", "Hoja de horas"));
    let y = m + 62.0;
    let half = (cw - 24.0) / 2.0;
    s.lined("Employee", lang.pick("Employee", "Empleado/a"), m, y, half);
    s.lined("Manager", lang.pick("Manager", "Responsable"), m + half + 24.0, y, half);
    s.lined_as("Week of", lang.pick("Week starting", "Semana del"), m, y + 38.0, half, Entry::Date);
    s.lined("Project", lang.pick("Project or client", "Proyecto o cliente"), m + half + 24.0, y + 38.0, half);

    // One row per day.
    let head = y + 96.0;
    let cols = [
        (m, cw * 0.18, Align::Left, lang.pick("Day", "Día")),
        (m + cw * 0.18, cw * 0.12, Align::Left, lang.pick("Start", "Entrada")),
        (m + cw * 0.30, cw * 0.12, Align::Left, lang.pick("End", "Salida")),
        (m + cw * 0.42, cw * 0.12, Align::Left, lang.pick("Break", "Pausa")),
        (m + cw * 0.54, cw * 0.12, Align::Right, lang.pick("Hours", "Horas")),
        (m + cw * 0.66, cw * 0.34, Align::Left, lang.pick("Notes", "Notas")),
    ];
    let top = header(s, head, &cols);
    let days = [
        lang.pick("Monday", "Lunes"),
        lang.pick("Tuesday", "Martes"),
        lang.pick("Wednesday", "Miércoles"),
        lang.pick("Thursday", "Jueves"),
        lang.pick("Friday", "Viernes"),
        lang.pick("Saturday", "Sábado"),
        lang.pick("Sunday", "Domingo"),
    ];
    let row = ((s.h - m - 170.0 - top) / 7.0).clamp(30.0, 46.0);
    for (i, day) in days.into_iter().enumerate() {
        let y = top + i as f64 * row;
        s.rule(m, m + cw, y + row, 0.5, RULE);
        s.text(m + 8.0, y + row / 2.0 + 3.5, Align::Left, Face::SansBold, 9.5, INK, 0.0, day);
        let fy = y + (row - 18.0) / 2.0;
        for (k, (x, cwid, align, label)) in cols.iter().enumerate().skip(1) {
            let name = format!("{day} {label}");
            let entry = if matches!(align, Align::Right) { Entry::Number } else { Entry::Text };
            let pad = if k == 5 { 8.0 } else { 4.0 };
            s.field(&name, x + 4.0, fy, cwid - pad - 4.0, 18.0, entry);
        }
    }

    // The total and the two signatures.
    let y = top + 7.0 * row + 18.0;
    s.panel(m + cw * 0.54, y, cw * 0.46, 34.0, 6.0, accent.tint(0.86));
    s.text(m + cw * 0.54 + 12.0, y + 21.5, Align::Left, Face::SansBold, 11.0, INK, 0.0, lang.pick("Total hours", "Total de horas"));
    s.field_sized("Total hours", m + cw * 0.54 + cw * 0.46 * 0.45, y + 7.0, cw * 0.46 * 0.55 - 12.0, 20.0, Entry::Number, 13.0);
    let y = s.h - m - 40.0;
    for (i, label) in
        [lang.pick("Employee signature", "Firma del empleado/a"), lang.pick("Manager signature", "Firma del responsable")].into_iter().enumerate()
    {
        let x = m + i as f64 * (half + 24.0);
        s.rule(x, x + half, y, 0.6, MUTED);
        s.label(x, y + 14.0, Align::Left, MUTED, label);
    }
}

pub(super) fn expenses(s: &mut Sheet, lang: Lang) {
    let (m, w) = (s.margin(), s.w);
    let cw = w - 2.0 * m;
    let accent = s.accent;

    s.title(m, m, lang.pick("Expense report", "Informe de gastos"));
    let y = m + 62.0;
    let half = (cw - 24.0) / 2.0;
    s.lined("Name", lang.pick("Name", "Nombre"), m, y, half);
    s.lined("Department", lang.pick("Department", "Departamento"), m + half + 24.0, y, half);
    s.lined("Period", lang.pick("Period", "Periodo"), m, y + 38.0, half);
    s.lined("Purpose", lang.pick("Purpose", "Motivo"), m + half + 24.0, y + 38.0, half);

    let head = y + 96.0;
    let cols = [
        (m, cw * 0.16, Align::Left, lang.pick("Date", "Fecha")),
        (m + cw * 0.16, cw * 0.40, Align::Left, lang.pick("Description", "Descripción")),
        (m + cw * 0.56, cw * 0.24, Align::Left, lang.pick("Category", "Categoría")),
        (m + cw * 0.80, cw * 0.20, Align::Right, lang.pick("Amount", "Importe")),
    ];
    let top = header(s, head, &cols);
    let categories: Vec<String> = [
        lang.pick("Travel", "Viajes"),
        lang.pick("Meals", "Comidas"),
        lang.pick("Lodging", "Alojamiento"),
        lang.pick("Supplies", "Material"),
        lang.pick("Other", "Otros"),
    ]
    .into_iter()
    .map(str::to_string)
    .collect();
    let rows = 12;
    let row = ((s.h - m - 150.0 - top) / rows as f64).clamp(24.0, 34.0);
    for i in 0..rows {
        let y = top + i as f64 * row;
        s.rule(m, m + cw, y + row, 0.5, RULE);
        let fy = y + (row - 18.0) / 2.0;
        let n = i + 1;
        s.field(&format!("{} {n}", lang.pick("Date", "Fecha")), m + 4.0, fy, cw * 0.16 - 8.0, 18.0, Entry::Date);
        s.field(&format!("{} {n}", lang.pick("Description", "Descripción")), m + cw * 0.16 + 4.0, fy, cw * 0.40 - 8.0, 18.0, Entry::Text);
        s.field(
            &format!("{} {n}", lang.pick("Category", "Categoría")),
            m + cw * 0.56 + 4.0,
            fy,
            cw * 0.24 - 8.0,
            18.0,
            Entry::Choice(categories.clone()),
        );
        s.field(&format!("{} {n}", lang.pick("Amount", "Importe")), m + cw * 0.80 + 4.0, fy, cw * 0.20 - 12.0, 18.0, Entry::Number);
    }

    let y = top + rows as f64 * row + 18.0;
    s.panel(m + cw * 0.56, y, cw * 0.44, 34.0, 6.0, accent.tint(0.86));
    s.text(m + cw * 0.56 + 12.0, y + 21.5, Align::Left, Face::SansBold, 11.0, INK, 0.0, lang.pick("Total", "Total"));
    s.field_sized("Total", m + cw * 0.56 + cw * 0.44 * 0.38, y + 7.0, cw * 0.44 * 0.62 - 12.0, 20.0, Entry::Number, 13.0);
    let y = s.h - m - 40.0;
    for (i, label) in [lang.pick("Submitted by", "Presentado por"), lang.pick("Approved by", "Aprobado por")].into_iter().enumerate() {
        let x = m + i as f64 * (half + 24.0);
        s.rule(x, x + half, y, 0.6, MUTED);
        s.label(x, y + 14.0, Align::Left, MUTED, label);
    }
}
