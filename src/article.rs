use super::error::KBGenError;
use super::Generateable;

use std::fs;
use std::path;

use clap::Clap;
use handlebars::Handlebars;
use serde::Serialize;

const TEMPLATE_NAME: &str = "article";
const TEMPLATE: &str = "{{> title }}

---

**Authors** - {{authors}}

**DOI** - [{{doi}}](https://doi.org/{{doi}})

---

---

**Notes**

**References**

---
";

/// Create a new article
#[derive(Clap, Debug, Serialize)]
pub struct Article {
    /// The article title
    #[clap(short)]
    title: String,
    /// The article authors
    #[clap(short)]
    authors: String,
    /// The article DOI, the link is later generated
    #[clap(short, long)]
    doi: String,
}

impl Generateable for Article {
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
