mod app;

use app::App;
use clap::Clap;

fn main() {
    let app = App::parse();
}
