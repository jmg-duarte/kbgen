pub(crate) mod app;

use anyhow::{Context, Result};
use app::{App, Command};

use clap::Clap;
use handlebars::Handlebars;

use kbgen::Generateable;

fn main() -> Result<()> {
    let app = App::parse();
    let mut hbs = Handlebars::new();
    match app.cmd {
        Command::Article(article) => article
            .generate(&app.destination, &mut hbs)
            .context("error while trying to generate article"),
        Command::Note(note) => note
            .generate(&app.destination, &mut hbs)
            .context("error while trying to generate note"),
    }
}
