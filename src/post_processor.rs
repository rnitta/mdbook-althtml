use regex::{Captures, Regex};
use std::collections::HashMap;

use mdbook::config::Playpen;
use mdbook::utils;

pub mod hbs_processor;

pub trait PostProcessor {
    fn execute(&self, html: &str) -> String;
}

// /////////// Ports of official built-in post-processors below //////////

/// Originally "build_header_links function"
/// Goes through the rendered HTML, making sure all header tags have
/// an anchor respectively so people can link to sections directly.
pub(crate) struct HeaderLinkProcessor;

impl PostProcessor for HeaderLinkProcessor {
    fn execute(&self, html: &str) -> String {
        let regex = Regex::new(r"<h(\d)>(.*?)</h\d>").unwrap();
        let mut id_counter = HashMap::new();

        regex
            .replace_all(html, |caps: &Captures<'_>| {
                let level = caps[1]
                    .parse()
                    .expect("Regex should ensure we only ever get numbers here");

                HeaderLinkProcessor::insert_link_into_header(level, &caps[2], &mut id_counter)
            })
            .into_owned()
    }
}

impl HeaderLinkProcessor {
    /// Blank initializer
    pub(crate) fn new() -> Self {
        Self {}
    }

    /// Insert a single link into a header, making sure each link gets its own
    /// unique ID by appending an auto-incremented number (if necessary).
    fn insert_link_into_header(
        level: usize,
        content: &str,
        id_counter: &mut HashMap<String, usize>,
    ) -> String {
        let raw_id = utils::id_from_content(content);

        let id_count = id_counter.entry(raw_id.clone()).or_insert(0);

        let id = match *id_count {
            0 => raw_id,
            other => format!("{}-{}", raw_id, other),
        };

        *id_count += 1;

        format!(
            r##"<h{level}><a class="header" href="#{id}" id="{id}">{text}</a></h{level}>"##,
            level = level,
            id = id,
            text = content
        )
    }
}

/// Originally "fix_code_blocks function"
// The rust book uses annotations for rustdoc to test code snippets,
// like the following:
// ```rust,should_panic
// fn main() {
//     // Code here
// }
// ```
// This function replaces all commas by spaces in the code block classes

pub(crate) struct CodeBlockProcessor;

impl CodeBlockProcessor {
    /// Blank initializer
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl PostProcessor for CodeBlockProcessor {
    fn execute(&self, html: &str) -> String {
        let regex = Regex::new(r##"<code([^>]+)class="([^"]+)"([^>]*)>"##).unwrap();
        regex
            .replace_all(html, |caps: &Captures<'_>| {
                let before = &caps[1];
                let classes = &caps[2].replace(",", " ");
                let after = &caps[3];

                format!(
                    r#"<code{before}class="{classes}"{after}>"#,
                    before = before,
                    classes = classes,
                    after = after
                )
            })
            .into_owned()
    }
}

/// Originally "add_playpen_pre function"
#[allow(dead_code)]
pub(crate) struct PlaypenProcessor {
    editable: bool,
    copy_js: bool,
}

impl PlaypenProcessor {
    pub(crate) fn new_with_config(playpen_config: &Playpen) -> Self {
        Self {
            editable: playpen_config.editable,
            copy_js: playpen_config.copy_js,
        }
    }

    fn partition_source(s: &str) -> (String, String) {
        let mut after_header = false;
        let mut before = String::new();
        let mut after = String::new();

        for line in s.lines() {
            let trimline = line.trim();
            let header = trimline.chars().all(char::is_whitespace) || trimline.starts_with("#![");
            if !header || after_header {
                after_header = true;
                after.push_str(line);
                after.push_str("\n");
            } else {
                before.push_str(line);
                before.push_str("\n");
            }
        }

        (before, after)
    }
}

impl PostProcessor for PlaypenProcessor {
    fn execute(&self, html: &str) -> String {
        let regex = Regex::new(r##"((?s)<code[^>]?class="([^"]+)".*?>(.*?)</code>)"##).unwrap();
        regex
            .replace_all(html, |caps: &Captures<'_>| {
                let text = &caps[1];
                let classes = &caps[2];
                let code = &caps[3];

                if (classes.contains("language-rust")
                    && !classes.contains("ignore")
                    && !classes.contains("noplaypen"))
                    || classes.contains("mdbook-runnable")
                {
                    // wrap the contents in an external pre block
                    if self.editable && classes.contains("editable")
                        || text.contains("fn main")
                        || text.contains("quick_main!")
                    {
                        format!("<pre class=\"playpen\">{}</pre>", text)
                    } else {
                        // we need to inject our own main
                        let (attrs, code) = Self::partition_source(code);

                        format!(
                            "<pre class=\"playpen\"><code class=\"{}\">\n# \
                             #![allow(unused_variables)]\n{}#fn main() {{\n{}#}}</code></pre>",
                            classes, attrs, code
                        )
                    }
                } else {
                    // not language-rust, so no-op
                    text.to_owned()
                }
            })
            .into_owned()
    }
}
