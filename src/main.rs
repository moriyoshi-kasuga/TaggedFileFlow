use std::io::{self, Write};
use std::{env, process::ExitCode};

use clap::Parser;
use color_print::cformat;
use commands::{Commands, Run};

mod commands;
mod data;

fn main() -> ExitCode {
    env::remove_var("RUST_LIB_BACKTRACE");
    env::remove_var("RUST_BACKTRACE");

    match Commands::parse().run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            _ = writeln!(io::stderr(), "{}", cformat!("<red>error:</> {:?}", e));
            ExitCode::FAILURE
        }
    }
}
