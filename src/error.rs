use std::path::StripPrefixError;
use thiserror::Error;

// TODO: do a pass on error messages and make sure they're decent
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum SnippextError {
    #[error("Config error: `{0}`")]
    ConfigError(#[from] config::ConfigError),

    /// Error variant that represents errors coming out of libgit2.
    #[error("Git error: `{0}`")]
    GitError(#[from] git2::Error),

    /// Error that may occur while I/O operations.
    #[error("IO error: `{0}`")]
    IoError(#[from] std::io::Error),

    /// Glob pattern error
    #[error("{0}")]
    GlobPatternError(String),

    #[error("Template render error: `{0}`")]
    TemplateRenderError(#[from] handlebars::RenderError),

    #[error("Template not found: `{0}`")]
    TemplateNotFound(String),

    #[error("Serde json error: `{0}`")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Snippet not found in file")]
    SnippetNotFound(),

    #[error("Stripe prefix error: `{0}`")]
    StripeError(#[from] StripPrefixError),

    /// Settings validation errors
    #[error("Settings error: `{0:?}`")]
    ValidationError(Vec<String>),
}
