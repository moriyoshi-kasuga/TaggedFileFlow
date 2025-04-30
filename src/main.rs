use std::{env, process::ExitCode};

use clap::Parser;
use commands::{Commands, Run};

mod commands;
mod data;

fn main() -> ExitCode {
    env::remove_var("RUST_LIB_BACKTRACE");
    env::remove_var("RUST_BACKTRACE");

    match Commands::parse().run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            color_print::ceprint!("<red>error</>: {:?}", e);
            ExitCode::FAILURE
        }
    }
}
