use clap::Clap;
use serde::Serialize;

pub(crate) const TEMPLATE_NAME: &str = "article";
pub(crate) const TEMPLATE: &str = "{{> title }}

---

**Authors** - {{authors}}

**DOI** - [{{doi}}](https://doi.org/{{doi}})

---

---

**Notes**

**References**

---";

/// Create a new article
#[derive(Clap, Debug, Serialize)]
pub struct Article {
    /// The article title
    #[clap(short)]
    pub(crate) title: String,
    /// The article authors
    #[clap(short)]
    pub(crate) authors: String,
    /// The article DOI, the link is later generated
    #[clap(short, long)]
    pub(crate) doi: String,
}

