use clap::Clap;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

#[derive(Clap)]
#[clap(version=VERSION, author=AUTHORS)]
pub struct App {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Clap)]
enum Command {
    #[clap(version=VERSION, author=AUTHORS)]
    Article(Article),
    #[clap(version=VERSION, author=AUTHORS)]
    Note(Note),
}

/// Create a new article
#[derive(Clap)]
struct Article {
    /// The article title
    title: String,
    /// The article DOI, the link is later generated
    doi: String,
}

/// Create a new note
#[derive(Clap)]
struct Note {
    /// The note title
    title: String
}
