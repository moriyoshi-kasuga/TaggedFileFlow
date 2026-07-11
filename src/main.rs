use std::process::ExitCode;

use clap::Parser;

use crate::cli::Cli;
use crate::model::Mode;

mod cli;
mod model;
mod ops;
mod shell;
mod store;
mod ui;

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            ui::report_error(&e);
            ExitCode::FAILURE
        }
    }
}

fn run(command: Cli) -> anyhow::Result<()> {
    match command {
        Cli::Mv(args) => ops::stash(Mode::Move, args),
        Cli::Cp(args) => ops::stash(Mode::Copy, args),
        Cli::Paste(args) => ops::paste(args),
        Cli::Del(args) => ops::del(args),
        Cli::List(args) => ops::list(args),
        Cli::Init(args) => ops::init(args),
    }
}
