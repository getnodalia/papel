//! Business documents: invoice, quote, receipt, letter, meeting minutes.

use super::Lang;
use super::sheet::{Align, Entry, Face, INK, MUTED, RULE, Rgb, Sheet};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Billing {
    Invoice,
    Quote,
}

pub(super) fn invoice(s: &mut Sheet, lang: Lang) {
    billing(s, lang, Billing::Invoice);
}

pub(super) fn quote(s: &mut Sheet, lang: Lang) {
    billing(s, lang, Billing::Quote);
}

/// The invoice and the quote share one page: who it is from and to, the items, the totals.
fn billing(s: &mut Sheet, lang: Lang, kind: Billing) {
    let (m, w) = (s.margin(), s.w);
    let cw = w - 2.0 * m;
    let accent = s.accent;
    let invoice = kind == Billing::Invoice;

    s.title(m, m, if invoice { lang.pick("Invoice", "Factura") } else { lang.pick("Quote", "Presupuesto") });

    // Number, date and the third date, stacked on the right.
    let mx = w - m - 168.0;
    s.lined(
        "Number",
        if invoice { lang.pick("Invoice number", "Número de factura") } else { lang.pick("Quote number", "Número de presupuesto") },
        mx,
        m - 6.0,
        168.0,
    );
    s.lined_as("Date", lang.pick("Date", "Fecha"), mx, m + 28.0, 168.0, Entry::Date);
    let third = if invoice { lang.pick("Due date", "Vencimiento") } else { lang.pick("Valid until", "Válido hasta") };
    s.lined_as("Due", third, mx, m + 62.0, 168.0, Entry::Date);

    // From and to.
    let top = m + 118.0;
    let gap = 20.0;
    let half = (cw - gap) / 2.0;
    for (i, (name, label)) in [("From", lang.pick("From", "De")), ("To", lang.pick("Bill to", "Facturar a"))].into_iter().enumerate() {
        let x = m + i as f64 * (half + gap);
        s.label(x, top, Align::Left, accent, label);
        s.writing_panel(name, x, top + 10.0, half, 80.0);
    }

    // Items.
    let head = top + 116.0;
    s.panel(m, head, cw, 24.0, 5.0, accent);
    let white = Rgb(1.0, 1.0, 1.0);
    let (qty_x, price_x, amount_x) = (m + cw * 0.50, m + cw * 0.62, m + cw * 0.81);
    s.label(m + 12.0, head + 15.5, Align::Left, white, lang.pick("Description", "Concepto"));
    s.label(price_x - 8.0, head + 15.5, Align::Right, white, lang.pick("Qty", "Cant."));
    s.label(amount_x - 8.0, head + 15.5, Align::Right, white, lang.pick("Unit price", "Precio"));
    s.label(m + cw - 12.0, head + 15.5, Align::Right, white, lang.pick("Amount", "Importe"));
    let row = 27.0;
    let rows = 8;
    for i in 0..rows {
        let y = head + 24.0 + i as f64 * row;
        s.rule(m, m + cw, y + row, 0.5, RULE);
        let n = i + 1;
        s.field(&format!("{} {n}", lang.pick("Description", "Concepto")), m + 8.0, y + 5.0, qty_x - m - 12.0, 18.0, Entry::Text);
        s.field(&format!("{} {n}", lang.pick("Quantity", "Cantidad")), qty_x, y + 5.0, price_x - qty_x - 8.0, 18.0, Entry::Number);
        s.field(&format!("{} {n}", lang.pick("Unit price", "Precio")), price_x, y + 5.0, amount_x - price_x - 8.0, 18.0, Entry::Number);
        s.field(&format!("{} {n}", lang.pick("Amount", "Importe")), amount_x, y + 5.0, m + cw - amount_x - 12.0, 18.0, Entry::Number);
    }

    // Totals, on the right.
    let after = head + 24.0 + rows as f64 * row + 16.0;
    let tx = m + cw * 0.55;
    let tw = m + cw - tx;
    let lines = [("Subtotal", lang.pick("Subtotal", "Subtotal")), ("Tax", lang.pick("Tax", "Impuestos"))];
    for (i, (name, label)) in lines.into_iter().enumerate() {
        let y = after + i as f64 * 26.0;
        s.text(tx + 12.0, y + 15.0, Align::Left, Face::Sans, 9.5, MUTED, 0.0, label);
        s.field(name, tx + tw * 0.45, y + 3.0, tw * 0.55 - 12.0, 18.0, Entry::Number);
    }
    let total_y = after + 2.0 * 26.0 + 2.0;
    s.panel(tx, total_y, tw, 34.0, 6.0, accent.tint(0.86));
    s.text(tx + 12.0, total_y + 21.5, Align::Left, Face::SansBold, 11.0, INK, 0.0, lang.pick("Total", "Total"));
    s.field_sized("Total", tx + tw * 0.40, total_y + 7.0, tw * 0.60 - 12.0, 20.0, Entry::Number, 13.0);

    // Notes, on the left; for a quote, the acceptance under the totals.
    let notes_h = (s.h - m - (after + 4.0)).max(80.0);
    let note_label =
        if invoice { lang.pick("Notes and payment details", "Notas y datos de pago") } else { lang.pick("Terms and notes", "Condiciones y notas") };
    s.label(m, after + 8.0, Align::Left, accent, note_label);
    s.writing_panel("Notes", m, after + 18.0, cw * 0.5 - 12.0, notes_h - 18.0);
    if !invoice {
        let y = total_y + 78.0;
        s.rule(tx, m + cw, y + 34.0, 0.6, MUTED);
        s.label(tx, y + 46.0, Align::Left, MUTED, lang.pick("Accepted by (signature and date)", "Aceptado por (firma y fecha)"));
    }
}

pub(super) fn receipt(s: &mut Sheet, lang: Lang) {
    let (m, w) = (s.margin(), s.w);
    let cw = w - 2.0 * m;
    let accent = s.accent;
    s.title(m, m, lang.pick("Receipt", "Recibo"));
    let mx = w - m - 168.0;
    s.lined("Number", lang.pick("Receipt number", "Número de recibo"), mx, m - 6.0, 168.0);
    s.lined_as("Date", lang.pick("Date", "Fecha"), mx, m + 28.0, 168.0, Entry::Date);

    // The amount, large, in a soft panel.
    let top = m + 100.0;
    s.panel(m, top, cw, 92.0, 10.0, accent.tint(0.88));
    s.label(m + 22.0, top + 28.0, Align::Left, accent, lang.pick("Amount received", "Importe recibido"));
    s.field_sized("Amount", m + 22.0, top + 38.0, cw - 44.0, 40.0, Entry::Text, 32.0);

    let y = top + 120.0;
    s.lined("Received from", lang.pick("Received from", "Recibido de"), m, y, cw);
    s.lined("For", lang.pick("For", "Concepto"), m, y + 46.0, cw);

    // How it was paid.
    let y = y + 104.0;
    s.section(lang.pick("Paid by", "Forma de pago"), m, y, cw);
    let options = [
        ("Cash", lang.pick("Cash", "Efectivo")),
        ("Card", lang.pick("Card", "Tarjeta")),
        ("Transfer", lang.pick("Bank transfer", "Transferencia")),
        ("Other", lang.pick("Other", "Otro")),
    ];
    for (i, (name, caption)) in options.into_iter().enumerate() {
        s.tick(name, caption, m + i as f64 * cw / 4.0, y + 24.0);
    }

    let y = y + 62.0;
    s.label(m, y, Align::Left, accent, lang.pick("Notes", "Notas"));
    let notes_h = ((s.h - m - 100.0) - (y + 10.0)).max(60.0);
    s.writing_panel("Notes", m, y + 10.0, cw, notes_h);

    // Signature.
    let y = s.h - m - 50.0;
    s.rule(m, m + cw * 0.5, y + 28.0, 0.6, MUTED);
    s.label(m, y + 42.0, Align::Left, MUTED, lang.pick("Received by (signature)", "Recibí (firma)"));
    s.lined_as("Signature date", lang.pick("Date", "Fecha"), m + cw * 0.62, y, cw * 0.38, Entry::Date);
}

pub(super) fn letter(s: &mut Sheet, lang: Lang) {
    let (m, w) = (s.margin(), s.w);
    let cw = w - 2.0 * m;
    let accent = s.accent;

    // Letterhead: who it is from, and the date.
    s.rect(m, m, 30.0, 4.0, accent);
    s.field_sized("Sender name", m, m + 14.0, cw * 0.58, 26.0, Entry::Text, 17.0);
    s.field("Sender address", m, m + 42.0, cw * 0.58, 44.0, Entry::Multiline);
    s.lined_as("Date", lang.pick("Date", "Fecha"), w - m - 150.0, m + 8.0, 150.0, Entry::Date);
    s.rule(m, m + cw, m + 102.0, 0.8, accent);

    // The recipient and the subject.
    s.label(m, m + 128.0, Align::Left, MUTED, lang.pick("To", "Para"));
    s.field("Recipient", m, m + 136.0, cw * 0.5, 58.0, Entry::Multiline);
    s.lined("Subject", lang.pick("Subject", "Asunto"), m, m + 212.0, cw);

    // The body fills what is left, and the closing sits under it.
    let body_top = m + 262.0;
    let close_top = s.h - m - 112.0;
    s.field("Letter body", m, body_top, cw, (close_top - body_top - 12.0).max(80.0), Entry::Multiline);
    s.text(m, close_top + 12.0, Align::Left, Face::Sans, 10.0, INK, 0.0, lang.pick("Sincerely,", "Atentamente,"));
    s.rule(m, m + cw * 0.42, close_top + 62.0, 0.6, MUTED);
    s.field("Signer", m, close_top + 66.0, cw * 0.42, 18.0, Entry::Text);
    s.label(m, close_top + 94.0, Align::Left, MUTED, lang.pick("Name and role", "Nombre y cargo"));
}

pub(super) fn minutes(s: &mut Sheet, lang: Lang) {
    let (m, w) = (s.margin(), s.w);
    let cw = w - 2.0 * m;
    let extra = (s.h - 792.0).max(0.0);

    s.title(m, m, lang.pick("Meeting minutes", "Acta de reunión"));
    let y = m + 62.0;
    s.lined("Topic", lang.pick("Topic", "Asunto"), m, y, cw);
    let third = (cw - 40.0) / 3.0;
    s.lined_as("Date", lang.pick("Date", "Fecha"), m, y + 38.0, third, Entry::Date);
    s.lined("Time", lang.pick("Time", "Hora"), m + third + 20.0, y + 38.0, third);
    s.lined("Place", lang.pick("Place", "Lugar"), m + 2.0 * (third + 20.0), y + 38.0, third);

    // Who was there, and who was not.
    let y = y + 92.0;
    let half = (cw - 20.0) / 2.0;
    s.section(lang.pick("Attendees", "Asistentes"), m, y, half);
    s.section(lang.pick("Apologies", "Ausentes"), m + half + 20.0, y, half);
    s.writing_panel("Attendees", m, y + 14.0, half, 60.0);
    s.writing_panel("Apologies", m + half + 20.0, y + 14.0, half, 60.0);

    let y = y + 100.0;
    s.section(lang.pick("Agenda and notes", "Orden del día y notas"), m, y, cw);
    s.writing_panel("Notes", m, y + 14.0, cw, 118.0 + extra * 0.6);

    let y = y + 156.0 + extra * 0.6;
    s.section(lang.pick("Decisions", "Decisiones"), m, y, cw);
    s.writing_panel("Decisions", m, y + 14.0, cw, 64.0 + extra * 0.2);

    // Who does what, by when.
    let y = y + 104.0 + extra * 0.2;
    s.section(lang.pick("Action items", "Acciones"), m, y, cw);
    let (owner_x, due_x) = (m + cw * 0.58, m + cw * 0.82);
    s.label(m, y + 26.0, Align::Left, MUTED, lang.pick("Action", "Acción"));
    s.label(owner_x, y + 26.0, Align::Left, MUTED, lang.pick("Owner", "Responsable"));
    s.label(due_x, y + 26.0, Align::Left, MUTED, lang.pick("Due", "Fecha límite"));
    let rows = ((s.h - m - (y + 34.0)) / 25.0).floor().clamp(2.0, 6.0) as usize;
    for i in 0..rows {
        let top = y + 34.0 + i as f64 * 25.0;
        s.rule(m, m + cw, top + 24.0, 0.5, RULE);
        let n = i + 1;
        s.field(&format!("{} {n}", lang.pick("Action", "Acción")), m, top + 4.0, owner_x - m - 10.0, 18.0, Entry::Text);
        s.field(&format!("{} {n}", lang.pick("Owner", "Responsable")), owner_x, top + 4.0, due_x - owner_x - 10.0, 18.0, Entry::Text);
        s.field(&format!("{} {n}", lang.pick("Due", "Fecha límite")), due_x, top + 4.0, m + cw - due_x, 18.0, Entry::Date);
    }
}
