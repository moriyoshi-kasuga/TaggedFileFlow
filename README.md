# tagged file flow

`tff` was created for people who find the mv and cp commands in the CLI a pain in the ass. (myself included).
You can save a file with a name, and then use [zoxide](https://github.com/ajeetdsouza/zoxide) or [fzf](https://github.com/junegunn/fzf) to quickly move the file to a new directory, paste the saved file, and you are done.
A quick directory move, paste in the saved files, and you're done.

## Getting started

```sh
mvg foo bar # foo and bar are stored with random characters
cpg copy.txt -n copy # Saved copy.txt under the name copy
# Change the directory with zoxide or something.
pasteg [random character] # moved foo and bar.
pasteg copy # copied copy.txt
# The process is now complete.
```

## Alias

<details>
<summary>To use alias, do the following</summary>

> Add this to the <ins>**end**</ins> of your config file (usually `~/.zshrc`):
>
> ```sh
> eval "$(tagged_file_flow init zsh)"
> ```

</details>

```sh
tagged_file_flow # default command
tff   # alias of tagged_file_flow
mvg   # alias of tff mv
cpg   # alias of tff cp
listg # alias of tff list
delg  # alias of tff del
```
