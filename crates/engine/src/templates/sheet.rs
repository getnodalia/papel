//! A page being drawn: the small set of marks every template is made of (text, rules, panels)
//! and the form fields that sit on top of them. Coordinates run from the top-left corner, in
//! points, so a layout reads the way it looks; they are turned into PDF user space here, once.

use papel_cos::{Dict, Document, Object, PdfString};
use papel_fonts::{Std14, literal, win_ansi, wrap_with};
use papel_forms::{BorderStyle, FieldFont, FieldProps, Look, NewField, add_field, set_props};

use crate::EditError;

/// A colour, each channel 0 to 1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Rgb(pub f64, pub f64, pub f64);

impl Rgb {
    /// Mixes `self` toward white by `amount` (0 keeps it, 1 is white).
    pub(crate) fn tint(self, amount: f64) -> Rgb {
        let m = |c: f64| c + (1.0 - c) * amount;
        Rgb(m(self.0), m(self.1), m(self.2))
    }
}

/// The quiet neutrals every template shares.
pub(crate) const INK: Rgb = Rgb(0.13, 0.15, 0.19);
pub(crate) const MUTED: Rgb = Rgb(0.42, 0.46, 0.53);
pub(crate) const RULE: Rgb = Rgb(0.82, 0.84, 0.88);
pub(crate) const WASH: Rgb = Rgb(0.955, 0.961, 0.973);

/// The faces a template may use: the standard fonts, which every viewer has.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Face {
    Sans,
    SansBold,
    SansOblique,
    Serif,
    SerifBold,
    SerifItalic,
}

impl Face {
    const ALL: [Face; 6] = [Face::Sans, Face::SansBold, Face::SansOblique, Face::Serif, Face::SerifBold, Face::SerifItalic];

    fn std(self) -> Std14 {
        match self {
            Face::Sans => Std14::Helvetica,
            Face::SansBold => Std14::HelveticaBold,
            Face::SansOblique => Std14::HelveticaOblique,
            Face::Serif => Std14::TimesRoman,
            Face::SerifBold => Std14::TimesBold,
            Face::SerifItalic => Std14::TimesItalic,
        }
    }

    /// The name the page's resources give this face.
    fn resource(self) -> &'static str {
        match self {
            Face::Sans => "T1",
            Face::SansBold => "T2",
            Face::SansOblique => "T3",
            Face::Serif => "T4",
            Face::SerifBold => "T5",
            Face::SerifItalic => "T6",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Align {
    Left,
    Center,
    Right,
}

/// What kind of field to lay on the page.
#[derive(Clone, Debug)]
pub(crate) enum Entry {
    Text,
    Multiline,
    /// A text field that right-aligns what is typed (amounts).
    Number,
    /// A single line, centred.
    Centered,
    /// Several lines, centred.
    CenteredMulti,
    Date,
    Check,
    Choice(Vec<String>),
}

#[derive(Clone, Debug)]
struct Field {
    name: String,
    rect: [f64; 4],
    entry: Entry,
    size: f64,
    font: FieldFont,
}

/// A page under construction.
pub(crate) struct Sheet {
    pub w: f64,
    pub h: f64,
    /// The template's own accent colour.
    pub accent: Rgb,
    content: Vec<u8>,
    fields: Vec<Field>,
}

fn n(v: f64) -> String {
    let v = if v.is_finite() { v } else { 0.0 };
    format!("{v:.2}")
}

impl Sheet {
    pub(crate) fn new(w: f64, h: f64, accent: Rgb) -> Self {
        Self { w, h, accent, content: Vec::new(), fields: Vec::new() }
    }

    /// The side margin: about 8.5% of the width (52 pt on Letter).
    pub(crate) fn margin(&self) -> f64 {
        (self.w * 0.085).round()
    }

    fn y(&self, y: f64) -> f64 {
        self.h - y
    }

    fn op(&mut self, s: &str) {
        self.content.extend_from_slice(s.as_bytes());
    }

    fn fill_colour(&mut self, c: Rgb) {
        self.op(&format!("{} {} {} rg\n", n(c.0), n(c.1), n(c.2)));
    }

    fn stroke_colour(&mut self, c: Rgb) {
        self.op(&format!("{} {} {} RG\n", n(c.0), n(c.1), n(c.2)));
    }

    /// A filled rectangle; `y` is its top edge.
    pub(crate) fn rect(&mut self, x: f64, y: f64, w: f64, h: f64, c: Rgb) {
        self.fill_colour(c);
        let py = self.y(y + h);
        self.op(&format!("{} {} {} {} re f\n", n(x), n(py), n(w), n(h)));
    }

    fn round_path(&mut self, x: f64, y: f64, w: f64, h: f64, r: f64) {
        let r = r.min(w / 2.0).min(h / 2.0).max(0.0);
        let k = r * 0.5523;
        let (x0, y0) = (x, self.y(y + h));
        let (x1, y1) = (x + w, self.y(y));
        self.op(&format!("{} {} m\n", n(x0 + r), n(y0)));
        self.op(&format!("{} {} l\n", n(x1 - r), n(y0)));
        self.op(&format!("{} {} {} {} {} {} c\n", n(x1 - r + k), n(y0), n(x1), n(y0 + r - k), n(x1), n(y0 + r)));
        self.op(&format!("{} {} l\n", n(x1), n(y1 - r)));
        self.op(&format!("{} {} {} {} {} {} c\n", n(x1), n(y1 - r + k), n(x1 - r + k), n(y1), n(x1 - r), n(y1)));
        self.op(&format!("{} {} l\n", n(x0 + r), n(y1)));
        self.op(&format!("{} {} {} {} {} {} c\n", n(x0 + r - k), n(y1), n(x0), n(y1 - r + k), n(x0), n(y1 - r)));
        self.op(&format!("{} {} l\n", n(x0), n(y0 + r)));
        self.op(&format!("{} {} {} {} {} {} c h\n", n(x0), n(y0 + r - k), n(x0 + r - k), n(y0), n(x0 + r), n(y0)));
    }

    /// A filled rounded rectangle.
    pub(crate) fn panel(&mut self, x: f64, y: f64, w: f64, h: f64, r: f64, c: Rgb) {
        self.fill_colour(c);
        self.round_path(x, y, w, h, r);
        self.op("f\n");
    }

    #[allow(clippy::too_many_arguments)]
    /// A rounded outline.
    pub(crate) fn outline(&mut self, x: f64, y: f64, w: f64, h: f64, r: f64, width: f64, c: Rgb) {
        self.stroke_colour(c);
        self.op(&format!("{} w\n", n(width)));
        self.round_path(x, y, w, h, r);
        self.op("S\n");
    }

    /// A straight rule.
    pub(crate) fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, width: f64, c: Rgb) {
        self.stroke_colour(c);
        let (py1, py2) = (self.y(y1), self.y(y2));
        self.op(&format!("{} w {} {} m {} {} l S\n", n(width), n(x1), n(py1), n(x2), n(py2)));
    }

    /// A horizontal rule from `x1` to `x2` at `y`.
    pub(crate) fn rule(&mut self, x1: f64, x2: f64, y: f64, width: f64, c: Rgb) {
        self.line(x1, y, x2, y, width, c);
    }

    /// The width of `s` set in `face` at `size`, with `tracking` points between letters.
    pub(crate) fn width(face: Face, size: f64, tracking: f64, s: &str) -> f64 {
        face.std().text_width(s, size) + tracking * s.chars().count() as f64
    }

    #[allow(clippy::too_many_arguments)]
    /// One line of text; `y` is its baseline.
    pub(crate) fn text(&mut self, x: f64, y: f64, align: Align, face: Face, size: f64, c: Rgb, tracking: f64, s: &str) {
        let w = Self::width(face, size, tracking, s);
        let left = match align {
            Align::Left => x,
            Align::Center => x - w / 2.0,
            Align::Right => x - w,
        };
        self.fill_colour(c);
        let py = self.y(y);
        self.op(&format!("BT /{} {} Tf {} Tc {} {} Td ", face.resource(), n(size), n(tracking), n(left), n(py)));
        self.content.extend(literal(&win_ansi(s)));
        self.op(" Tj ET\n");
    }

    /// A small upper-case label with room between the letters.
    pub(crate) fn label(&mut self, x: f64, y: f64, align: Align, c: Rgb, s: &str) {
        self.text(x, y, align, Face::SansBold, 7.2, c, 1.1, &s.to_uppercase());
    }

    #[allow(clippy::too_many_arguments)]
    /// A paragraph wrapped to `width`, first baseline at `y`. Returns the baseline after the last line.
    pub(crate) fn paragraph(&mut self, x: f64, y: f64, width: f64, face: Face, size: f64, leading: f64, c: Rgb, s: &str) -> f64 {
        let mut at = y;
        for line in wrap_with(s, size, width, |t, sz| face.std().text_width(t, sz)) {
            self.text(x, at, Align::Left, face, size, c, 0.0, &line);
            at += leading;
        }
        at
    }

    /// A form field over `rect` (`x`, `y` top, `w`, `h`), named `name`.
    pub(crate) fn field(&mut self, name: &str, x: f64, y: f64, w: f64, h: f64, entry: Entry) {
        self.field_set(name, x, y, w, h, entry, 10.0, FieldFont::Helvetica);
    }

    #[allow(clippy::too_many_arguments)]
    /// [`Self::field`] with its own type size, in Helvetica.
    pub(crate) fn field_sized(&mut self, name: &str, x: f64, y: f64, w: f64, h: f64, entry: Entry, size: f64) {
        self.field_set(name, x, y, w, h, entry, size, FieldFont::Helvetica);
    }

    #[allow(clippy::too_many_arguments)]
    /// [`Self::field`] with its own type size, in Times.
    pub(crate) fn field_serif(&mut self, name: &str, x: f64, y: f64, w: f64, h: f64, entry: Entry, size: f64) {
        self.field_set(name, x, y, w, h, entry, size, FieldFont::Times);
    }

    #[allow(clippy::too_many_arguments)]
    fn field_set(&mut self, name: &str, x: f64, y: f64, w: f64, h: f64, entry: Entry, size: f64, font: FieldFont) {
        let (bottom, top) = (self.y(y + h), self.y(y));
        self.fields.push(Field { name: name.to_string(), rect: [x, bottom, x + w, top], entry, size, font });
    }

    /// A small filled diamond centred on (`x`, `y`).
    pub(crate) fn diamond(&mut self, x: f64, y: f64, r: f64, c: Rgb) {
        self.fill_colour(c);
        let py = self.y(y);
        self.op(&format!("{} {} m {} {} l {} {} l {} {} l h f\n", n(x - r), n(py), n(x), n(py + r), n(x + r), n(py), n(x), n(py - r)));
    }

    /// The finished document: one page, its fonts, its fields and its title.
    pub(crate) fn finish(self, title: &str, ink: Rgb) -> Result<Document, EditError> {
        let mut faces = Dict::new();
        for face in Face::ALL {
            let mut font = Dict::new();
            font.set(b"Type".to_vec(), Object::name("Font"));
            font.set(b"Subtype".to_vec(), Object::name("Type1"));
            font.set(b"BaseFont".to_vec(), Object::name(face.std().base_font()));
            font.set(b"Encoding".to_vec(), Object::name("WinAnsiEncoding"));
            faces.set(face.resource().as_bytes().to_vec(), Object::Dict(font));
        }
        let mut resources = Dict::new();
        resources.set(b"Font".to_vec(), Object::Dict(faces));
        let mut doc = papel_create::from_contents(&[(self.w, self.h, resources, self.content)])?;
        let look = Look { border: None, fill: None, width: 0.0, style: BorderStyle::Solid, text: [ink.0, ink.1, ink.2], font: FieldFont::Helvetica };
        for f in &self.fields {
            let kind = match &f.entry {
                Entry::Text | Entry::Number | Entry::Centered => NewField::Text { multiline: false },
                Entry::Multiline | Entry::CenteredMulti => NewField::Text { multiline: true },
                Entry::Date => NewField::Date,
                Entry::Check => NewField::CheckBox,
                Entry::Choice(options) => NewField::Combo { options: options.clone(), editable: false },
            };
            let name = add_field(&mut doc, 0, f.rect, &kind, Some(&f.name))?;
            let mut props = FieldProps { look: Some(Look { font: f.font, ..look }), ..Default::default() };
            match f.entry {
                Entry::Number => props.quadding = Some(2),
                Entry::Centered | Entry::CenteredMulti => props.quadding = Some(1),
                Entry::Check => props.look = Some(Look { border: Some([ink.0, ink.1, ink.2]), width: 0.75, ..look }),
                _ => {}
            }
            if !matches!(f.entry, Entry::Check) {
                props.font_size = Some(f.size);
            }
            set_props(&mut doc, &name, &props)?;
        }
        let mut info = Dict::new();
        info.set(b"Title".to_vec(), PdfString::text(title));
        info.set(b"Producer".to_vec(), PdfString::text("Papel"));
        let r = doc.add(info);
        doc.trailer_mut().set(b"Info".to_vec(), Object::Ref(r));
        Ok(doc)
    }
}
