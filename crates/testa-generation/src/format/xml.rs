use testa_interpreter::{generator::Record, object::Object};

use crate::generator::{FileConfig, FileGenerator};

#[derive(Debug)]
pub struct XmlGenerator {
    root_element: String,
    record_element: String,
}

impl XmlGenerator {
    pub fn new() -> Self {
        Self {
            root_element: "data".to_string(),
            record_element: "record".to_string(),
        }
    }

    pub fn from_config(config: &FileConfig) -> Self {
        let root_element = config
            .get("root")
            .and_then(|o| match o {
                Object::String(s) => Some(s.clone()),
                _ => None,
            })
            .unwrap_or_else(|| "data".to_string());

        let record_element = config
            .get("record")
            .and_then(|o| match o {
                Object::String(s) => Some(s.clone()),
                _ => None,
            })
            .unwrap_or_else(|| "record".to_string());

        Self {
            root_element,
            record_element,
        }
    }

    fn escape_xml(&self, value: &str) -> String {
        value
            .replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("'", "&apos;")
    }
}

impl FileGenerator for XmlGenerator {
    fn generate(&self, record: &Record) -> Result<String, crate::error::GeneratorError> {
        let mut xml = format!("  <{}>", self.record_element);

        for (key, value) in record {
            xml.push_str(&format!(
                "\n    <{}>{}</{}>",
                key,
                self.escape_xml(&format!("{}", value)),
                key
            ));
        }

        xml.push_str(&format!("\n  </{}>\n", self.record_element));

        Ok(xml)
    }

    fn extension(&self) -> &'static str {
        "xml"
    }

    fn generate_header(&self, _fields: &[String]) -> Option<String> {
        Some(format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<{}>",
            self.root_element
        ))
    }

    fn generate_footer(&self) -> Option<String> {
        Some(format!("</{}>", self.root_element))
    }
}

impl Default for XmlGenerator {
    fn default() -> Self {
        XmlGenerator::new()
    }
}
