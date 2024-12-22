use crate::config::KdlFmtConfig;

#[inline]
pub fn run() -> std::io::Result<()> {
    let config = KdlFmtConfig::default();

    let mut doc = kdl::KdlDocument::new();

    let mut indent_size_node = kdl::KdlNode::new(KdlFmtConfig::indent_size_key());
    indent_size_node.push(kdl::KdlEntry::from(kdl::KdlValue::Integer(
        config.indent.len() as i128,
    )));
    doc.nodes_mut().push(indent_size_node);

    let mut use_tab_node = kdl::KdlNode::new(KdlFmtConfig::use_tabs_key());
    use_tab_node.push(kdl::KdlEntry::from(kdl::KdlValue::Bool(config.use_tabs)));
    doc.nodes_mut().push(use_tab_node);

    let autoformat_config = kdl::FormatConfig::builder().indent(&config.indent).build();
    doc.autoformat_config(&autoformat_config);

    std::fs::write(KdlFmtConfig::filename(), doc.to_string())
}
