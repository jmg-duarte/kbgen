use clap::Clap;

/// Create a new note
#[derive(Clap)]
pub struct Note {
    /// The note title
    pub title: String
}