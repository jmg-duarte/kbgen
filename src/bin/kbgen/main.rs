pub(crate) mod app;

use crate::app::{App, Command};
use kbgen::Generator;
use std::convert::TryFrom;

use std::fs;
use std::path;

use anyhow::{Context, Result};
use clap::Clap;

fn main() -> Result<()> {
    let app = App::parse();
    let output = match app.cmd {
        Command::Article(article) => {
            let generator = Generator::try_from(article)
                .context("error while building the article generator")?;
            generator.generate()
        }
        Command::Note(note) => {
            let generator =
                Generator::try_from(note).context("error while building the note generator")?;
            generator.generate()
        }
    }?;

    let dest_path = path::Path::new(&app.destination);
    if dest_path.exists() {
        Err(kbgen::error::KBGenError::FileExists(app.destination).into())
    } else {
        fs::write(dest_path, output).context(format!(
            "error while writing the file to {}",
            app.destination
        ))?;
        Ok(())
    }
}
