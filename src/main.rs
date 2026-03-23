use std::process::ExitCode;

use clap::Parser;
use commands::{Commands, Run};

mod commands;
mod data;

fn main() -> ExitCode {
    match Commands::parse().run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            color_print::ceprintln!("<red>error</>: {:#}", e);
            ExitCode::FAILURE
        }
    }
}
