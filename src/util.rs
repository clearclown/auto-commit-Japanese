// src/util.rs

use rand::seq::SliceRandom;
use spinners::{Spinner, Spinners};
use crate::cli::Cli;

pub fn start_spinner(cli: &Cli, msg: &str) -> Option<Spinner> {
    if cli.dry_run || !cli.verbose.is_silent() { return None; }
    let mut rng = rand::thread_rng();
    let sp = [
        Spinners::Earth, Spinners::Aesthetic, Spinners::BoxBounce,
        Spinners::BouncingBar, Spinners::Flip, Spinners::Layer,
    ].choose(&mut rng).cloned().unwrap();
    Some(Spinner::new(sp, msg.into()))
}

pub fn stop_spinner(sp: Option<Spinner>, msg: &str) {
    if let Some(mut s) = sp { s.stop_with_message(msg.into()); }
}
