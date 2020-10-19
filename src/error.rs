use thiserror::Error;

use handlebars::{RenderError, TemplateError};

#[derive(Error, Debug)]
pub enum KBGenError {
    #[error("file '{0}' already exists")]
    FileExists(String),

    #[error(transparent)]
    Template(#[from] TemplateError),

    #[error(transparent)]
    Render(#[from] RenderError),

    #[error(transparent)]
    Io(#[from] std::io::Error)
}
