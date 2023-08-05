// TODO: this might not be needed
pub const DEFAULT_SNIPPEXT_CONFIG: &str = include_str!("./default_snippext_config.yaml");
pub const DEFAULT_CONFIG: &'static str = "snippext";
pub const DEFAULT_COMMENT_PREFIXES: &'static [&'static str] = &["// ", "# ", "<!-- "];
pub const DEFAULT_BEGIN: &'static str = "snippet::";
pub const DEFAULT_END: &'static str = "end::";
pub const DEFAULT_INCLUDE: &'static str = "snippet::include::";
// snippet::start::
// snippet::end::
pub const DEFAULT_REPLACE: &'static str = "snippet::replace::"; // TODO: do we want this?
pub const DEFAULT_TEMPLATE: &'static str = "{{snippet}}";
pub const DEFAULT_FILE_EXTENSION: &'static str = "md";
pub const DEFAULT_SOURCE_FILES: &'static str = "**";
pub const DEFAULT_OUTPUT_DIR: &'static str = "./snippets/";
pub const SNIPPEXT_TEMPLATE_ATTRIBUTE: &'static str = "snippext_template";
pub const DEFAULT_TEMPLATE_IDENTIFIER: &'static str = "default";
