use clap::Parser;
use tagged_file_flow::{
    commands::{del, init, list, paste, save},
    data::Save,
    types::{Args, Commands},
};

fn main() {
    match Args::parse().cmd {
        Commands::MV(args) => save::action(Save::MV, args),
        Commands::CP(args) => save::action(Save::CP, args),
        Commands::Paste(args) => paste::action(args),
        Commands::Del(args) => del::action(args),
        Commands::List(args) => list::action(args),
        Commands::Init => init::action(),
    }
}
