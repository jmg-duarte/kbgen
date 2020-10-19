use super::error::KBGenError;
use super::Generateable;

use std::fs;
use std::path;

use clap::Clap;
use handlebars::Handlebars;
use serde::Serialize;

const TEMPLATE_NAME: &str = "article";
const TEMPLATE: &str = "{{> title }}";

/// Create a new note
#[derive(Clap, Debug, Serialize)]
pub struct Note {
    /// The note title
    #[clap(short)]
    pub title: String,
}

impl Generateable for Note {
    fn generate(&self, destination: &str, hbs: &mut Handlebars) -> Result<(), KBGenError> {
        self.setup(hbs)?;
        hbs.register_template_string(TEMPLATE_NAME, TEMPLATE)?;
        let path = path::Path::new(&destination);
        if path.exists() {
            return Err(KBGenError::FileExists(destination.to_string()));
        }

        let render_output = hbs.render(TEMPLATE_NAME, self)?;
        fs::write(path, render_output)?;
        Ok(())
    }
}
