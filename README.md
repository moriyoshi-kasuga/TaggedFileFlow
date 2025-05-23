# tagged file flow

`tff` is designed for those who find the mv and cp commands in the CLI frustrating (myself included).
It simplifies file management by allowing you to save a file under a random characters or specific name
and then use tools like [zoxide](https://github.com/ajeetdsouza/zoxide) or [fzf](https://github.com/junegunn/fzf) to quickly navigate directories.
Once there, you can paste the saved file into the target location —quick and efficient.

## Getting started

```sh
mvg sample.txt sample_folder # sample.txt and sample_folder are stored with random characters
cpg copy.txt -n copy # Saved copy.txt under the name copy
# Change directory using zoxide, fzf, etc.
pasteg [random character] # moved sample.txt and sample_folder to the current directory
pasteg copy # copied copy.txt
# The process is now complete.
```

[Sample](https://github.com/user-attachments/assets/bc682bb0-42d4-417e-9701-0964f60c14ab)

## Installation

- cargo: `cargo install --git https://github.com/moriyoshi-kasuga/TaggedFileFlow`

## Alias

```sh
tagged_file_flow # default command
tff   # alias of tagged_file_flow
mvg   # alias of tff mv
cpg   # alias of tff cp
listg # alias of tff list
delg  # alias of tff del
```

<details open>
<summary>To use alias, do the following</summary>

> Add this to your config file (usually `~/.zshrc` on zsh, `~/.bashrc` on bash, etc.)
> supports zsh, bash, fish, and nushell
>
> ```sh
> eval "$(tagged_file_flow init zsh)"
> ```

</details>
