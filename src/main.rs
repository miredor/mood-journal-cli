mod app;
mod journal;
mod prompts;

use app::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    if let Err(err) = app::run(cli) {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
