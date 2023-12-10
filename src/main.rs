mod layout;
mod page;

use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

use crate::page::PageViewBuilder;

fn main() {
    let page_width = 210.0;
    let page_height = 297.0;

    let (doc, initial_page, _) = PdfDocument::new("CV", Mm(page_width), Mm(page_height), "Layer 1");

    let mut view_builder = PageViewBuilder::new();
    view_builder.page_size(page_width, page_height);
    view_builder.load_main_font(
        12.0,
        2.0,
        "assets/fonts/static/SometypeMono-Regular.ttf",
        &doc,
    );
    view_builder.load_auxiliary_fonts(
        "assets/fonts/static/SometypeMono-Bold.ttf",
        "assets/fonts/static/SometypeMono-Italic.ttf",
        "assets/fonts/static/SometypeMono-BoldItalic.ttf",
        &doc,
    );
    let view = view_builder.build();

    let current_page = doc.get_page(initial_page);

    view.draw_page(&current_page, |mut ctx| {
        ctx.frame(|mut ctx| {
            ctx.vsplit(1, |mut ctx| {
                ctx.padding(1, 1, 0, 0, |mut ctx| {
                    ctx.ftext("<h1><bo>Title");
                });
            }, |mut ctx| {
                ctx.ftext("<it>Lorem ipsum<fg> dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.");
            });
        });
    });

    doc.save(&mut BufWriter::new(File::create("out.pdf").unwrap())).unwrap();
}


