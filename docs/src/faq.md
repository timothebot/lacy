# Frequently Asked Questions

For other questions, see bottom on how to contact me.

- [General Questions](#general-questions)
  - [Why not use cd + autocomplete?](#why-not-use-cd--autocomplete)
  - [Why not use Z (zoxide or similar tools)?](#why-not-use-z-zoxide-or-similar-tools)
- [Technical Questions](#technical-questions)
  - [Can I change the default lacy key?](#can-i-change-the-default-lacy-key)
  - [Can I use my own selector instead of the built in one?](#can-i-use-my-own-selector-instead-of-the-built-in-one)
  - [Why does it need a shell script?](#why-does-it-need-a-shell-script)
  - [How can I use both Lacy and `z`/`zoxide`?](#how-can-i-use-both-lacy-and-zzoxide)
- [Other Questions](#other-questions)

## General Questions

### Why not use cd + autocomplete?

Autocomplete is great, but sometimes it can get annoying with lots of similar named folders.

```text
cwd/
  folder_ab/ <-- Target
  folder_ba/ 
  folder_ac/
  ab_folder/
```

In this scenario, you'd have to press tab multiple times and see what was autocompleted. With lacy, you can just type `y foab`.

In some cases, autocomplete is still useful. Lacy still offers that autocomplete.

### Why not use Z (zoxide or similar tools)?

These tools are great and a great inspiration for lacy, but the goal of lacy is to have a tool that you can use from the first second and in unknown environments, which e.g. Zoxide doesn't, as it first has to learn your behavior.

## Technical Questions

### Can I change the default lacy key?

Default is `y`, but you can easily change it by modifying the `lacy init shell` command.
You can change it by passing the `--cmd` option, e.g. `lacy init zsh --cmd c` sets it to `c`.

See [shell options](./setup.md#shell-options) for all available options.

### Can I use my own selector instead of the built in one?

Yes! For example, you may want to use [fzf](https://github.com/junegunn/fzf).
You can easily do that by passing the `--custom-fuzzy` option to `lacy init shell`, e.g. `lacy init zsh --custom-fuzzy fzf`.

See [shell options](./setup.md#shell-options) for all available options.

### Why does it need a shell script?

It is needed because you can't change the directory without using `cd`. So the shell script just executes `cd` if needed.

### How can I use both Lacy and `z`/`zoxide`?

If you have `cd` aliased to `z`, then it should work as long as the Lacy shell eval is below the `z` eval.
If not, run `lacy init <shell>` and manually add the result to your shell config. Then, replace the `cd`'s with `z`.

## Other Questions

Feel free to open an issue, contact me on discord (@tiimo, DM me, don't send friend requests).
