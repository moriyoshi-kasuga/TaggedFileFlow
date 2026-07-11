# tagged file flow

`tff` is designed for those who find the mv and cp commands in the CLI frustrating (myself included).
Tag files right where they are, jump around with tools like [zoxide](https://github.com/ajeetdsouza/zoxide) or [fzf](https://github.com/junegunn/fzf),
then paste the tagged files into the target directory — quick and efficient.

## Getting started

```sh
mvg sample.txt sample_folder # tagged under a random name (e.g. "abc"), moved on paste
cpg copy.txt -n copy         # tagged under the name "copy", copied on paste
# Change directory using zoxide, fzf, etc.
pasteg abc  # moves sample.txt and sample_folder into the current directory
pasteg copy # copies copy.txt into the current directory
# The process is now complete.
```

[Sample](https://github.com/user-attachments/assets/bc682bb0-42d4-417e-9701-0964f60c14ab)

## Installation

```sh
cargo install --git https://github.com/moriyoshi-kasuga/TaggedFileFlow
```

This installs a single binary named `tff`.

## Commands

| Command | Description |
| --- | --- |
| `tff mv <paths>...` | Tag files/folders to be **moved** on paste |
| `tff cp <paths>...` | Tag files/folders to be **copied** on paste |
| `tff paste <names>...` | Transfer tagged files into the current directory |
| `tff list [names]...` | Show all tags, or only the named ones |
| `tff del <names>...` | Delete tags |
| `tff init <shell>` | Print alias definitions for your shell |

- `mv` / `cp` accept `-n <name>` to name the tag; without it a random 3-letter name is generated.
- `paste` refuses to overwrite existing paths unless `--force` is given. With `--force`, a colliding path is replaced wholesale (not merged).
- A tag is consumed by `paste`.

## Aliases

`tff init` prints short aliases for the subcommands:

| Alias | Expands to |
| --- | --- |
| `mvg` | `tff mv` |
| `cpg` | `tff cp` |
| `pasteg` | `tff paste` |
| `delg` | `tff del` |
| `listg` | `tff list` |

<details open>
<summary>To use the aliases, do the following</summary>

> Add this to your shell config file (usually `~/.zshrc` on zsh, `~/.bashrc` on bash, etc.)
> Supported shells: `bash`, `posix`, `zsh`, `fish`, and `nushell`
>
> ```sh
> eval "$(tff init zsh)"
> ```

</details>
