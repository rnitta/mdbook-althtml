use mdbook_althtml::post_processor::hbs_processor::HbsProcessor;
use mdbook::MDBook;
use mdbook_althtml::HtmlHandlebars;
use std::path::Path;

fn main() {
    let handlebar_postprocessor = HbsProcessor::new();
    let mut altrenderer = HtmlHandlebars::new();
    altrenderer.change_output_dir("html");
    altrenderer.add_post_processor(handlebar_postprocessor);

    let root_dir = Path::new("./");
    let mut book = MDBook::load(root_dir).expect("invalid root_dir");

    book
        .with_renderer(altrenderer)
        .build()
        .expect("Book generation failed");
}
