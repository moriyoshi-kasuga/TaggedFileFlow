use clap::{Parser, ValueEnum};

use super::Run;

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum Shell {
    Bash,
    Posix,
    Zsh,
    Fish,
    Nushell,
}

#[derive(Parser)]
#[command(about)]
pub struct Init {
    pub shell: Shell,
}

impl Run for Init {
    fn run(self) -> anyhow::Result<()> {
        const SH: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/tff.sh"));
        const NU: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/tff.nu"));
        const FISH: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/tff.fish"));

        let shell: &str = match self.shell {
            Shell::Bash => SH,
            Shell::Posix => SH,
            Shell::Zsh => SH,
            Shell::Fish => FISH,
            Shell::Nushell => NU,
        };

        println!("{shell}");

        Ok(())
    }
}
