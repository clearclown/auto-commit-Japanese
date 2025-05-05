// src/cli.rs

use clap::{Parser};
use clap_verbosity_flag::{Verbosity, InfoLevel};
use log::error;

#[derive(Parser, Debug)]
#[command(name = "Auto Commit", version, author, about)]
pub struct Cli {
    #[clap(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    #[arg(long = "dry-run", help = "Output the generated message, but don't create a commit.")]
    pub dry_run: bool,

    #[arg(short, long, help = "Edit the generated commit message before committing.")]
    pub review: bool,

    #[arg(short, long, help = "Don't ask for confirmation before committing.")]
    pub force: bool,
}

impl Cli {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}

pub fn init_logger(cli: &Cli) {
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();
    if std::env::var("OPENAI_API_KEY").is_err() {
        error!("OPENAI_API_KEY not set"); std::process::exit(1);
    }
}
