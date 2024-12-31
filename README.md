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

> Add this to your config file (usually `~/.zshrc` or `~/.bashrc`):
>
> ```sh
> eval "$(tagged_file_flow init zsh)"
> ```

</details>
