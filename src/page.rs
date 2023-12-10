use std::cell::RefCell;
use printpdf::*;
use std::fs::File;
use std::io::Read;
use std::iter;
use std::rc::Rc;
use azul_text_layout::text_layout::{split_text_into_words, words_to_scaled_words};
use azul_text_layout::text_shaping::get_font_metrics_freetype;

use crate::layout::LayoutContext;

pub struct PageView {
    font: IndirectFontRef,
    font_bold: IndirectFontRef,
    font_italic: IndirectFontRef,
    font_bold_italic: IndirectFontRef,
    page_width: f32,
    page_height: f32,
    font_size: f32,
    character_spacing: f32,
    view_symbol_width: usize,
    view_symbol_height: usize,
    view_padding_width: f32,
    view_padding_height: f32,
    bg_color: (f32, f32, f32),
    fg_color: (f32, f32, f32),
    h1_color: (f32, f32, f32),
    h2_color: (f32, f32, f32),
    h3_color: (f32, f32, f32),
    h4_color: (f32, f32, f32)
}

impl PageView {
    pub(crate) fn draw_page(&self, page: &PdfPageReference, layout: fn(LayoutContext)) {
        let bg = page.add_layer("bg");
        let fg = page.add_layer("fg");

        let buf: Vec<Vec<Symbol>> = iter::repeat(iter::repeat(Symbol {
            character: ' ',
            color: self.fg_color,
            bold: false,
            italic: false
        }).take(self.view_symbol_width).collect()).take(self.view_symbol_height).collect();
        let buf = Rc::new(RefCell::new(buf));
        layout(LayoutContext::new(
            buf.clone(),
            (self.view_symbol_width, self.view_symbol_height),
            (0, 0),
            (self.view_symbol_width, self.view_symbol_height),
            self.fg_color,
            self.h1_color,
            self.h2_color,
            self.h3_color,
            self.h4_color
        ));

        bg.set_fill_color(Color::Rgb(Rgb::new(self.bg_color.0, self.bg_color.1, self.bg_color.2, None)));

        bg.add_polygon(Polygon {
            rings: vec![vec![(Point::new(Mm(0.0), Mm(0.0)), false),
                             (Point::new(Mm(self.page_width), Mm(0.0)), false),
                             (Point::new(Mm(self.page_width), Mm(self.page_height)), false),
                             (Point::new(Mm(0.0), Mm(self.page_height)), false)]],
            mode: PolygonMode::Fill,
            winding_order: WindingOrder::NonZero,
        });

        fg.begin_text_section();

        fg.set_text_cursor(Mm(self.view_padding_width), Mm(self.page_height) - Mm::from(Pt(self.font_size)) - Mm(self.view_padding_height));
        fg.set_line_height(self.font_size + self.character_spacing);
        fg.set_character_spacing(self.character_spacing);
        fg.set_text_rendering_mode(TextRenderingMode::Fill);

        for line in buf.borrow_mut().iter() {
            for c in line {
                fg.set_fill_color(Color::Rgb(Rgb::new(c.color.0, c.color.1, c.color.2, None)));
                match (c.bold, c.italic) {
                    (false, false) => {
                        fg.set_font(&self.font, self.font_size);
                        fg.write_text(c.character, &self.font)
                    },
                    (true, false) => {
                        fg.set_font(&self.font_bold, self.font_size);
                        fg.write_text(c.character, &self.font_bold)
                    },
                    (false, true) => {
                        fg.set_font(&self.font_italic, self.font_size);
                        fg.write_text(c.character, &self.font_italic)
                    },
                    (true, true) => {
                        fg.set_font(&self.font_bold_italic, self.font_size);
                        fg.write_text(c.character, &self.font_bold_italic)
                    },
                }
            }
            fg.add_line_break();
        }

        fg.end_text_section();
    }
}

#[derive(Clone, Copy)]
pub struct Symbol {
    pub character: char,
    pub color: (f32, f32, f32),
    pub bold: bool,
    pub italic: bool,
}

pub struct PageViewBuilder {
    page_width: f32,
    page_height: f32,
    character_spacing: f32,
    font_size: f32,
    page_padding: f32,
    symbol_width: Option<f32>,
    symbol_height: Option<f32>,
    font: Option<IndirectFontRef>,
    font_bold: Option<IndirectFontRef>,
    font_italic: Option<IndirectFontRef>,
    font_bold_italic: Option<IndirectFontRef>,
    bg_color: (f32, f32, f32),
    fg_color: (f32, f32, f32),
    h1_color: (f32, f32, f32),
    h2_color: (f32, f32, f32),
    h3_color: (f32, f32, f32),
    h4_color: (f32, f32, f32)
}

impl PageViewBuilder {
    pub fn new() -> Self {
        Self {
            page_width: 210.0,
            page_height: 297.0,
            character_spacing: 2.0,
            font_size: 12.0,
            page_padding: 10.0,
            symbol_width: None,
            symbol_height: None,
            font: None,
            font_bold: None,
            font_italic: None,
            font_bold_italic: None,
            bg_color: (0.0, 0.0, 0.0),
            fg_color: (1.0, 1.0, 1.0),
            h1_color: (1.0, 0.0, 0.0),
            h2_color: (0.0, 1.0, 0.0),
            h3_color: (0.0, 0.0, 1.0),
            h4_color: (1.0, 1.0, 0.0)
        }
    }

    pub fn background_color(&mut self, r: f32, g: f32, b: f32) {
        self.bg_color = (r, g, b);
    }

    pub fn default_text_color(&mut self, r: f32, g: f32, b: f32) {
        self.fg_color = (r, g, b);
    }

    pub fn highlight_1_color(&mut self, r: f32, g: f32, b: f32) {
        self.h1_color = (r, g, b);
    }

    pub fn highlight_2_color(&mut self, r: f32, g: f32, b: f32) {
        self.h2_color = (r, g, b);
    }

    pub fn highlight_3_color(&mut self, r: f32, g: f32, b: f32) {
        self.h3_color = (r, g, b);
    }

    pub fn highlight_4_color(&mut self, r: f32, g: f32, b: f32) {
        self.h4_color = (r, g, b);
    }

    pub fn page_size(&mut self, w: f32, h: f32) {
        self.page_width = w;
        self.page_height = h;
    }

    pub fn load_main_font(&mut self, font_size: f32, character_spacing: f32, font: &str, doc: &PdfDocumentReference) {
        self.font = Some(doc.add_external_font(File::open(font).unwrap()).unwrap());

        self.font_size = font_size;
        self.character_spacing = character_spacing;

        let symbol_size = Self::calculate_symbol_size(font, font_size, character_spacing);

        self.symbol_width = Some(symbol_size.0);
        self.symbol_height = Some(symbol_size.1);
    }

    pub fn load_auxiliary_fonts(&mut self, font_bold: &str, font_italic: &str, font_bold_italic: &str, doc: &PdfDocumentReference) {
        self.font_bold = Some(doc.add_external_font(File::open(font_bold).unwrap()).unwrap());
        self.font_italic = Some(doc.add_external_font(File::open(font_italic).unwrap()).unwrap());
        self.font_bold_italic = Some(doc.add_external_font(File::open(font_bold_italic).unwrap()).unwrap());
    }

    pub fn build(self) -> PageView {
        let (view_symbol_width, view_symbol_height) = Self::calculate_view_symbol_size(self.page_width, self.page_height, self.page_padding, self.symbol_width.expect("fonts required"), self.symbol_height.expect("fonts required"));
        let (view_width, view_height) = Self::calculate_view_size(view_symbol_width, view_symbol_height, self.symbol_width.expect("fonts required"), self.symbol_height.expect("fonts required"), self.character_spacing);
        let (view_padding_width, view_padding_height) = Self::calculate_view_padding(view_width, view_height, self.page_width, self.page_height);

        PageView {
            font: self.font.expect("fonts required"),
            font_bold: self.font_bold.expect("aux fonts required"),
            font_italic: self.font_italic.expect("aux fonts required"),
            font_bold_italic: self.font_bold_italic.expect("aux fonts required"),
            page_width: self.page_width,
            page_height: self.page_height,
            font_size: self.font_size,
            character_spacing: self.character_spacing,
            view_symbol_width,
            view_symbol_height,
            view_padding_width,
            view_padding_height,
            bg_color: self.bg_color,
            fg_color: self.fg_color,
            h1_color: self.h1_color,
            h2_color: self.h2_color,
            h3_color: self.h3_color,
            h4_color: self.h4_color
        }
    }
}

impl PageViewBuilder {
    fn calculate_symbol_size(path: &str, font_size: f32, character_spacing: f32) -> (f32, f32) {
        let mut font = Vec::new();
        File::open(path).unwrap().read_to_end(&mut font).expect("Font file not found");
        let font_metrics = get_font_metrics_freetype(&font, 0);
        let words = split_text_into_words("-");
        let scaled_words = words_to_scaled_words(&words, &font, 0, font_metrics, font_size * 96.0 / 72.0);
        let font_width = scaled_words.longest_word_width * 72.0 / 96.0;

        (Mm::from(Pt(font_width + character_spacing)).0, Mm::from(Pt(font_size + character_spacing)).0)
    }

    fn calculate_view_symbol_size(page_width: f32, page_height: f32, page_padding: f32, symbol_width: f32, symbol_height: f32) -> (usize, usize) {
        let view_symbol_width = ((page_width - 2.0 * page_padding) / symbol_width).floor() as usize;
        let view_symbol_height = ((page_height - 2.0 * page_padding) / symbol_height).floor() as usize;

        (view_symbol_width, view_symbol_height)
    }

    fn calculate_view_size(view_symbol_width: usize, view_symbol_height: usize, symbol_width: f32, symbol_height: f32, character_spacing: f32) -> (f32, f32) {
        (view_symbol_width as f32 * symbol_width, view_symbol_height as f32 * symbol_height + character_spacing)
    }

    fn calculate_view_padding(view_width: f32, view_height: f32, page_width: f32, page_heigth: f32) -> (f32, f32) {
        ((page_width - view_width) / 2.0, (page_heigth - view_height) / 2.0)
    }
}