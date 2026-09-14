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

pub(crate) const NAME_PRIORITY_ENV: &str = "TFF_NAME_PRIORITY";

pub(crate) fn script(shell: Shell, priority: Option<&str>) -> String {
    let render: fn(&str, &str) -> String = match shell {
        Shell::Bash | Shell::Posix | Shell::Zsh => {
            |name, sub| format!("alias {name}='{BIN_NAME} {sub}'")
        }
        Shell::Fish => |name, sub| format!("alias {name} '{BIN_NAME} {sub}'"),
        Shell::Nushell => {
            |name, sub| format!("def {name} [...args] {{ {BIN_NAME} {sub} ...$args }}")
        }
    };
    let mut lines: Vec<String> = ALIASES
        .iter()
        .map(|(name, sub)| render(name, sub))
        .collect();
    if let Some(priority) = priority {
        lines.push(priority_export(shell, priority));
    }
    lines.join("\n")
}

fn priority_export(shell: Shell, priority: &str) -> String {
    match shell {
        Shell::Bash | Shell::Posix | Shell::Zsh => {
            format!("export {NAME_PRIORITY_ENV}={priority}")
        }
        Shell::Fish => format!("set -gx {NAME_PRIORITY_ENV} {priority}"),
        Shell::Nushell => format!("$env.{NAME_PRIORITY_ENV} = \"{priority}\""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scripts_alias_every_subcommand() {
        assert!(script(Shell::Zsh, None).contains("alias mvg='tff mv'"));
        assert!(script(Shell::Fish, None).contains("alias pasteg 'tff paste'"));
        assert!(script(Shell::Nushell, None).contains("def delg [...args] { tff del ...$args }"));
    }

    #[test]
    fn scripts_without_priority_omit_the_env_var() {
        assert!(!script(Shell::Zsh, None).contains(NAME_PRIORITY_ENV));
    }

    #[test]
    fn scripts_export_priority_per_shell() {
        assert!(script(Shell::Zsh, Some("jkl")).contains("export TFF_NAME_PRIORITY=jkl"));
        assert!(script(Shell::Fish, Some("jkl")).contains("set -gx TFF_NAME_PRIORITY jkl"));
        assert!(
            script(Shell::Nushell, Some("jkl")).contains("$env.TFF_NAME_PRIORITY = \"jkl\"")
        );
    }
}
