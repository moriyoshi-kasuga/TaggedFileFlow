use clap::ValueEnum;

use crate::cli::BIN_NAME;

#[derive(ValueEnum, Clone, Copy, Debug)]
pub(crate) enum Shell {
    Bash,
    Posix,
    Zsh,
    Fish,
    Nushell,
}

const ALIASES: &[(&str, &str)] = &[
    ("mvg", "mv"),
    ("cpg", "cp"),
    ("pasteg", "paste"),
    ("delg", "del"),
    ("listg", "list"),
];

pub(crate) fn script(shell: Shell) -> String {
    let render: fn(&str, &str) -> String = match shell {
        Shell::Bash | Shell::Posix | Shell::Zsh => {
            |name, sub| format!("alias {name}='{BIN_NAME} {sub}'")
        }
        Shell::Fish => |name, sub| format!("alias {name} '{BIN_NAME} {sub}'"),
        Shell::Nushell => {
            |name, sub| format!("def {name} [...args] {{ {BIN_NAME} {sub} ...$args }}")
        }
    };
    ALIASES
        .iter()
        .map(|(name, sub)| render(name, sub))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scripts_alias_every_subcommand() {
        assert!(script(Shell::Zsh).contains("alias mvg='tff mv'"));
        assert!(script(Shell::Fish).contains("alias pasteg 'tff paste'"));
        assert!(script(Shell::Nushell).contains("def delg [...args] { tff del ...$args }"));
    }
}
