use kdl::{FormatConfig, KdlDocument};

use crate::error::KdlFmtError;

#[derive(Debug)]
pub struct KdlFmtConfig {
    pub indent: u8,
}

impl Default for KdlFmtConfig {
    fn default() -> Self {
        KdlFmtConfig {
            indent: FormatConfig::default()
                .indent
                .len()
                .max(0)
                .min(u8::MAX as usize) as u8,
        }
    }
}

impl KdlFmtConfig {
    const fn filename() -> &'static str {
        "kdlfmt.kdl"
    }

    fn parse_config(config: &str) -> miette::Result<KdlDocument> {
        let c = kdl::KdlDocument::parse_v1(&config)?;
        Ok(c)
    }

    pub fn load() -> Result<Self, KdlFmtError> {
        let mut config = KdlFmtConfig::default();

        if let Ok(config_str) = std::fs::read_to_string(Self::filename()) {
            // TODO: custom parse error
            let doc = Self::parse_config(&config_str)
                .map_err(|error| KdlFmtError::ParseError(None, error))?;

            if let Some(indent) = doc.get_arg("indent") {
                if let Some(count) = indent.as_integer() {
                    config.indent = count.max(0).min(u8::MAX as i128) as u8;
                }
            }
        }

        Ok(config)
    }

    pub fn get_indent(&self) -> String {
        let mut spaces = String::new();

        for _ in 0..self.indent {
            spaces.push(' ');
        }

        spaces
    }

    pub fn init() -> std::io::Result<()> {
        let config = Self::default();

        let mut doc = kdl::KdlDocument::new();

        let mut indent_node = kdl::KdlNode::new("indent");
        indent_node.push(kdl::KdlEntry::from(kdl::KdlValue::Integer(i128::from(
            config.indent,
        ))));
        doc.nodes_mut().push(indent_node);

        let indent_spaces = config.get_indent();
        let autoformat_config = kdl::FormatConfig::builder().indent(&indent_spaces).build();
        doc.autoformat_config(&autoformat_config);

        std::fs::write(Self::filename(), &doc.to_string())
    }
}
