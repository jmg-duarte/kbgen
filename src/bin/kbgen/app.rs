use clap::Clap;

use kbgen::article::Article;
use kbgen::note::Note;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

#[derive(Clap)]
#[clap(version=VERSION, author=AUTHORS)]
pub struct App {
    /// The file destination
    pub destination: String,
    #[clap(subcommand)]
    pub cmd: Command,
}

#[derive(Clap)]
pub enum Command {
    #[clap(version=VERSION, author=AUTHORS)]
    Article(Article),
    #[clap(version=VERSION, author=AUTHORS)]
    Note(Note),
}
