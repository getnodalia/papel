//! Ready-made documents to start from (Create ▸ Templates): an invoice, a letter, a timesheet,
//! a certificate and the like. Each is one page drawn here, in English or Spanish, in the page
//! size the caller asks for, with real form fields laid over the design so it can be filled in
//! straight away. Nothing is read from disk: a template is code, so it is the same everywhere
//! and carries no third-party artwork or fonts (only the standard fonts every viewer has).

use papel_cos::Document;

use crate::EditError;

mod business;
mod forms;
mod parts;
mod personal;
mod sheet;

use sheet::{Rgb, Sheet};

/// The language a template is written in. Papel ships these two; any other interface language
/// gets the English text.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Lang {
    #[default]
    En,
    Es,
}

impl Lang {
    /// The language for a code such as `es`, `es-MX` or `en_US`; English for anything else.
    pub fn parse(code: &str) -> Lang {
        let primary = code.trim().split(['-', '_']).next().unwrap_or("");
        if primary.eq_ignore_ascii_case("es") { Lang::Es } else { Lang::En }
    }

    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Es => "es",
        }
    }

    pub(crate) fn pick(self, en: &'static str, es: &'static str) -> &'static str {
        match self {
            Lang::En => en,
            Lang::Es => es,
        }
    }
}

/// The groups the gallery shows its templates in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Category {
    Business,
    Forms,
    Personal,
}

impl Category {
    pub const ALL: [Category; 3] = [Category::Business, Category::Forms, Category::Personal];

    pub fn label(self, lang: Lang) -> &'static str {
        match self {
            Category::Business => lang.pick("Business", "Negocio"),
            Category::Forms => lang.pick("Forms", "Formularios"),
            Category::Personal => lang.pick("Personal", "Personal"),
        }
    }
}

/// One entry of the gallery.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TemplateInfo {
    /// The stable English id scripts and the `create.template` command use.
    pub id: &'static str,
    pub category: Category,
    pub name: &'static str,
    pub summary: &'static str,
}

/// Draws a template onto a page of the given size.
type Draw = fn(&mut Sheet, Lang);

struct Template {
    id: &'static str,
    category: Category,
    name: [&'static str; 2],
    summary: [&'static str; 2],
    /// The template's own accent: a muted tone, each one calm beside the others.
    accent: Rgb,
    /// Landscape pages (a certificate).
    landscape: bool,
    draw: Draw,
}

pub(crate) const SLATE: Rgb = Rgb(0.33, 0.40, 0.53);
pub(crate) const SAGE: Rgb = Rgb(0.31, 0.49, 0.42);
pub(crate) const CLAY: Rgb = Rgb(0.66, 0.40, 0.32);
pub(crate) const OCEAN: Rgb = Rgb(0.20, 0.45, 0.58);
pub(crate) const PLUM: Rgb = Rgb(0.47, 0.34, 0.52);
pub(crate) const OCHRE: Rgb = Rgb(0.69, 0.50, 0.18);

const TEMPLATES: &[Template] = &[
    Template {
        id: "invoice",
        category: Category::Business,
        name: ["Invoice", "Factura"],
        summary: ["Itemised, with totals and payment notes", "Con conceptos, totales y notas de pago"],
        accent: SLATE,
        landscape: false,
        draw: business::invoice,
    },
    Template {
        id: "quote",
        category: Category::Business,
        name: ["Quote", "Presupuesto"],
        summary: ["An offer with terms and a place to accept it", "Una oferta con condiciones y espacio para aceptarla"],
        accent: OCEAN,
        landscape: false,
        draw: business::quote,
    },
    Template {
        id: "receipt",
        category: Category::Business,
        name: ["Receipt", "Recibo"],
        summary: ["Proof of payment, with the way it was paid", "Justificante de pago, con la forma de pago"],
        accent: SAGE,
        landscape: false,
        draw: business::receipt,
    },
    Template {
        id: "letter",
        category: Category::Business,
        name: ["Letter", "Carta"],
        summary: ["A clean letterhead and a page to write on", "Un membrete limpio y una página para escribir"],
        accent: SLATE,
        landscape: false,
        draw: business::letter,
    },
    Template {
        id: "minutes",
        category: Category::Business,
        name: ["Meeting minutes", "Acta de reunión"],
        summary: ["Attendees, decisions and who does what by when", "Asistentes, decisiones y quién hace qué y cuándo"],
        accent: PLUM,
        landscape: false,
        draw: business::minutes,
    },
    Template {
        id: "registration",
        category: Category::Forms,
        name: ["Registration form", "Formulario de inscripción"],
        summary: ["Contact details, choices and a signature", "Datos de contacto, opciones y firma"],
        accent: OCEAN,
        landscape: false,
        draw: forms::registration,
    },
    Template {
        id: "timesheet",
        category: Category::Forms,
        name: ["Timesheet", "Hoja de horas"],
        summary: ["A week of hours, day by day", "Una semana de horas, día a día"],
        accent: SAGE,
        landscape: false,
        draw: forms::timesheet,
    },
    Template {
        id: "expenses",
        category: Category::Forms,
        name: ["Expense report", "Informe de gastos"],
        summary: ["Itemised expenses with a total to approve", "Gastos detallados con un total para aprobar"],
        accent: CLAY,
        landscape: false,
        draw: forms::expenses,
    },
    Template {
        id: "resume",
        category: Category::Personal,
        name: ["Résumé", "Currículum"],
        summary: ["A calm two-column page about you", "Una página tranquila en dos columnas sobre ti"],
        accent: SLATE,
        landscape: false,
        draw: personal::resume,
    },
    Template {
        id: "planner",
        category: Category::Personal,
        name: ["Weekly planner", "Planificador semanal"],
        summary: ["Seven days and a space for what matters", "Siete días y un espacio para lo importante"],
        accent: OCHRE,
        landscape: false,
        draw: personal::planner,
    },
    Template {
        id: "checklist",
        category: Category::Personal,
        name: ["Checklist", "Lista de tareas"],
        summary: ["Tick things off, with room for notes", "Marca tareas, con espacio para notas"],
        accent: SAGE,
        landscape: false,
        draw: personal::checklist,
    },
    Template {
        id: "notes",
        category: Category::Personal,
        name: ["Study notes", "Apuntes de estudio"],
        summary: ["Cues, notes and a summary, in three zones", "Palabras clave, apuntes y resumen, en tres zonas"],
        accent: OCEAN,
        landscape: false,
        draw: personal::notes,
    },
    Template {
        id: "certificate",
        category: Category::Personal,
        name: ["Certificate", "Certificado"],
        summary: ["A landscape page to recognise an achievement", "Una página apaisada para reconocer un logro"],
        accent: OCHRE,
        landscape: true,
        draw: personal::certificate,
    },
];

/// The words the gallery around the templates uses, in the language of the templates themselves.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Words {
    pub heading: &'static str,
    pub intro: &'static str,
    pub use_template: &'static str,
}

pub fn words(lang: Lang) -> Words {
    Words {
        heading: lang.pick("Start from a template", "Empieza desde una plantilla"),
        intro: lang.pick(
            "Calm, ready-made pages you can fill in right away. Each one is a normal PDF with real form fields.",
            "Páginas tranquilas y listas para rellenar al momento. Cada una es un PDF normal con campos de formulario reales.",
        ),
        use_template: lang.pick("Use template", "Usar plantilla"),
    }
}

/// Every template, in gallery order, named and described in `lang`.
pub fn list(lang: Lang) -> Vec<TemplateInfo> {
    let at = |pair: [&'static str; 2]| if lang == Lang::Es { pair[1] } else { pair[0] };
    TEMPLATES.iter().map(|t| TemplateInfo { id: t.id, category: t.category, name: at(t.name), summary: at(t.summary) }).collect()
}

/// The ids of every template, in gallery order.
pub fn ids() -> Vec<&'static str> {
    TEMPLATES.iter().map(|t| t.id).collect()
}

/// The template `id` as a one-page document on a page of `page` (width, height) points. A landscape
/// template turns the page on its side. Fields are laid out over the design, so the document is
/// ready to fill in.
pub fn build(id: &str, lang: Lang, page: (f64, f64)) -> Result<Document, EditError> {
    let template = TEMPLATES.iter().find(|t| t.id == id).ok_or_else(|| {
        let known = ids().join(", ");
        EditError::Invalid(format!("no template named {id:?}; the templates are {known}"))
    })?;
    let (w, h) = page;
    if !(w.is_finite() && h.is_finite() && (400.0..=5_000.0).contains(&w.min(h)) && (560.0..=5_000.0).contains(&w.max(h))) {
        return Err(EditError::Invalid("a template needs a page of at least 400 x 560 points (and at most 5000 on a side)".into()));
    }
    let (w, h) = if template.landscape { (w.max(h), w.min(h)) } else { (w.min(h), w.max(h)) };
    let mut sheet = Sheet::new(w, h, template.accent);
    (template.draw)(&mut sheet, lang);
    sheet.finish(template.name[usize::from(lang == Lang::Es)], sheet::INK)
}

#[cfg(test)]
mod tests;
