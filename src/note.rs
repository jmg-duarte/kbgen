
use clap::Clap;
use serde::Serialize;

pub(crate) const TEMPLATE_NAME: &str = "article";
pub(crate) const TEMPLATE: &str = "{{> title }}";

/// Create a new note
#[derive(Clap, Debug, Serialize)]
pub struct Note {
    /// The note title
    #[clap(short)]
    pub(crate) title: String,
}
