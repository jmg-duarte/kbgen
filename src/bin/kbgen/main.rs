pub(crate) mod app;

use app::{App, Command};
use clap::Clap;
use handlebars::Handlebars;

use kbgen::Generateable;

fn main() {
    let app = App::parse();
    let mut hbs = Handlebars::new();
    match app.cmd {
        Command::Article(article) => article.generate(&app.destination, &mut hbs),
        Command::Note(note) => {}
    }
}
