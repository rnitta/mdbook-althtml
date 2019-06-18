/// An alternative of the official mdbook html renderer.
/// Using handlebars is in common.
/// A main difference is that this renderer have more modifiability.
///
/// Official mdbook html renderer processes like:
///     [compiling md -> rendering handlebars -> post-processing -> output html files ]
///
/// This renderer processes like:
///     [compiling md] -> (former post-processing) -> (rendering handlebars) -> (latter post-processing) -> [output html files]
///
/// The steps enclosed in `[]` are not modifiable, and `()` means modifiable steps.
///
pub use self::hbs_renderer::HtmlHandlebars;
pub use self::post_processor::PostProcessor;

mod hbs_renderer;
mod helpers;
pub mod post_processor;

#[cfg(feature = "search")]
mod search;
