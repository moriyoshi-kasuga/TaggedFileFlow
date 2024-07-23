use clap::Parser;
use tagged_file_flow::{
    commands::{del, init, paste, save, show},
    data::Save,
    types::{Args, Commands},
};

fn main() {
    match Args::parse().cmd {
        Commands::MV(args) => save::action(Save::MV, args),
        Commands::CP(args) => save::action(Save::CP, args),
        Commands::Paste(args) => paste::action(args),
        Commands::Del => del::action(),
        Commands::List => show::action(),
        Commands::Init => init::action(),
    }
}
