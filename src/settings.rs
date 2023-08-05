use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::constants::{
    DEFAULT_BEGIN, DEFAULT_COMMENT_PREFIXES, DEFAULT_END, DEFAULT_FILE_EXTENSION,
    DEFAULT_OUTPUT_DIR, DEFAULT_SOURCE_FILES, DEFAULT_TEMPLATE,
};
use crate::templates::SnippextTemplate;
use crate::types::{LinkFormat, SnippetSource};
use crate::SnippextResult;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SnippextSettings {
    pub begin: String,
    pub end: String,
    pub extension: String,
    pub comment_prefixes: HashSet<String>,
    pub templates: HashMap<String, SnippextTemplate>,
    pub sources: Vec<SnippetSource>,
    pub output_dir: Option<String>,
    pub targets: Option<Vec<String>>,
    pub link_format: Option<LinkFormat>,
    pub url_prefix: Option<String>,
}

impl SnippextSettings {
    /// Create default SnippextSettings which will have the following
    /// begin: [`DEFAULT_BEGIN`]
    /// end: [`DEFAULT_END`]
    /// extension: [`DEFAULT_FILE_EXTENSION`]
    /// comment_prefixes: [`DEFAULT_COMMENT_PREFIXES`]
    /// template: [`DEFAULT_TEMPLATE`]
    /// sources: all files via [`DEFAULT_SOURCE_FILES`] glob
    /// output_dir: [`DEFAULT_OUTPUT_DIR`]
    pub fn default() -> Self {
        Self {
            begin: String::from(DEFAULT_BEGIN),
            end: String::from(DEFAULT_END),
            extension: String::from(DEFAULT_FILE_EXTENSION),
            comment_prefixes: DEFAULT_COMMENT_PREFIXES
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            templates: HashMap::from([(
                String::from("default"),
                SnippextTemplate {
                    content: String::from(DEFAULT_TEMPLATE),
                    default: true,
                },
            )]),
            sources: vec![SnippetSource::new_local(vec![String::from(
                DEFAULT_SOURCE_FILES,
            )])],
            output_dir: Some(String::from(DEFAULT_OUTPUT_DIR)),
            targets: None,
            link_format: None,
            url_prefix: None,
        }
    }

    /// Create SnippextSettings from config file
    ///
    /// # Arguments
    ///
    /// * `path` - Path of config file
    pub fn from_config<S: AsRef<Path>>(path: S) -> SnippextResult<Self> {
        let content = fs::read_to_string(path)?;
        let settings = serde_json::from_str(content.as_str())?;
        Ok(settings)
    }

    // TODO: <S: Into<String>>
    pub fn new(
        comment_prefixes: HashSet<String>,
        begin: String,
        end: String,
        extension: String,
        templates: HashMap<String, SnippextTemplate>,
        sources: Vec<SnippetSource>,
        output_dir: Option<String>,
        targets: Option<Vec<String>>,
        link_format: Option<LinkFormat>,
        url_prefix: Option<String>,
    ) -> Self {
        Self {
            begin,
            end,
            extension,
            comment_prefixes,
            templates,
            sources,
            output_dir,
            targets,
            link_format,
            url_prefix,
        }
    }
}
