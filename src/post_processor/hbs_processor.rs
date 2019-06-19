/// Just an example of post-processor.
/// You can develop your post-processor with the pub trait PostProcessor.
use crate::PostProcessor;
use handlebars::Handlebars;
use serde_json::{json, Value};
use std::fs;
use toml;

/// Post-processor using handlebars.
pub struct HbsProcessor {
    dict: Value,
}

impl HbsProcessor {
    /// Add simple key-value rule. Nesting is not permitted.
    pub fn add_rule(&mut self, k: &str, v: &str) {
        if k.contains('.') || k.contains(' ') {
            panic!(format!(
                r#"The key "{}" cannnot contain "." nor " (whitespace)""#,
                k
            ));
        }
        self.merge(&json!({ k: v }));
    }

    /// Load and merge rules from JSON file.
    pub fn load_json(&mut self, path: &str) {
        let text = fs::read_to_string(path)
            .unwrap_or_else(|_| panic!(format!(r#""{}" doesn't seems valid path."#, path)));
        let value = serde_json::from_str(&text).unwrap();
        self.merge(&value);
    }

    /// Load and merge rules from toml file.
    pub fn load_toml(&mut self, path: &str) {
        let file = fs::read_to_string(path)
            .unwrap_or_else(|_| panic!(format!(r#""{}" doesn't seems valid path."#, path)));
        let toml_value = file.parse::<toml::Value>().unwrap();
        let value: Value =
            serde_json::from_str(&serde_json::to_string_pretty(&toml_value).unwrap()).unwrap();
        self.merge(&value);
    }

    /// Merge new rules to dict. Used for adding/loading rules.
    fn merge(&mut self, other: &Value) {
        Self::_merge(&mut self.dict, other);
    }

    // thanks ;) https://git.io/fjagY
    fn _merge(a: &mut Value, b: &Value) {
        match (a, b) {
            (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
                for (k, v) in b {
                    Self::_merge(a.entry(k.clone()).or_insert(Value::Null), v);
                }
            }
            (a, b) => {
                *a = b.clone();
            }
        }
    }
}

impl Default for HbsProcessor {
    /// Default initializer
    fn default() -> Self {
        Self { dict: json!({}) }
    }
}

impl PostProcessor for HbsProcessor {
    fn execute(&self, html: &str) -> String {
        Handlebars::default()
            .render_template(&html, &self.dict)
            .unwrap_or_else(|_| panic!("Exception at hbs post-processor."))
            .to_owned()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge() {
        let mut processor = HbsProcessor::default();
        processor.merge(&json!({}));
        // blank + blank = blank
        assert_eq!(processor.dict, json!({}));
        processor.merge(&json!({"a": "1"}));
        // add kv
        assert_eq!(processor.dict, json!({"a": "1"}));
        processor.merge(&json!({"b": "2"}));
        // add kv to existing dict
        assert_eq!(processor.dict, json!({"a": "1", "b": "2"}));
        processor.merge(&json!({"t": {"c": "3"}}));
        // add nested kv
        assert_eq!(processor.dict, json!({"a": "1", "b": "2", "t": {"c": "3"}}));
        processor.merge(&json!({"t": {"c": "4"}}));
        // override nested kv
        assert_eq!(processor.dict, json!({"a": "1", "b": "2", "t": {"c": "4"}}));
    }

    #[test]
    fn test_add_rule() {
        let mut processor = HbsProcessor::default();
        processor.add_rule("hoge", "fuga");
        assert_eq!(processor.dict, json!({"hoge": "fuga"}));
        processor.add_rule("foo", "bar");
        assert_eq!(processor.dict, json!({"hoge": "fuga", "foo": "bar"}));
    }

    #[test]
    fn test_load_json() {
        let mut processor = HbsProcessor::default();
        processor.load_json("./tests/1.json");
        assert_eq!(processor.dict, json!({"a": "b"}));
        processor.load_json("./tests/2.json");
        assert_eq!(
            processor.dict,
            json!({"a": "b", "c": "d", "e": { "f": "g" }})
        );
    }

    #[test]
    fn test_load_toml() {
        let mut processor = HbsProcessor::default();
        processor.load_toml("./tests/1.toml");
        assert_eq!(processor.dict, json!({"a": "b"}));
        processor.load_toml("./tests/2.toml");
        assert_eq!(
            processor.dict,
            json!({"a": "b", "c": "d", "e": { "f": "g" }})
        );
    }
}
