
<div align="center">
<img src="./docs/assets/lacy_logo.png" width="150">
<h1>Lacy</h1>

Lacy is a **magical cd alternative**  that makes navigating much more efficient!
<br>
<br>
<sup>
⭐ Consider starring this repo – your support motivates me a lot! ⭐
</sup>
<div>
<a href="https://crates.io/crates/lacy">
  <img alt="Crates.io Version" src="https://img.shields.io/crates/v/lacy">
</a>
<a href="https://crates.io/crates/lacy">
  <img alt="Crates.io Total Downloads" src="https://img.shields.io/crates/d/lacy">
</a>
<a href="https://github.com/timothebot/lacy/">
  <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/timothebot/lacy">
</a>
</div>

<br>
<a href="https://lacy.tiimo.space/install">Install</a>
&nbsp;~&nbsp;
<a href="https://lacy.tiimo.space/setup">Setup</a>
&nbsp;~&nbsp;
<a href="https://lacy.tiimo.space/faq">FAQ</a>
&nbsp;~&nbsp;
<a href="https://lacy.tiimo.space/">Documentation</a>
<br>
<br>

<img src="docs/assets/showcase.png">

</div>

Not convinced yet? Check out [all features](https://lacy.tiimo.space/features)!

---

## Install

### Cargo (Universal)

```sh
cargo install lacy
```

### Homebrew (macOS/Linux)

```sh
brew install timothebot/tap/lacy
```

### AUR (Arch Linux)

```sh
yay -S lacy # or whatever aur helper you're using
```

### NixOS / Nix Flake

See in the [documentation](https://lacy.tiimo.space/install.html#nixos-).

## Setup

After you installed lacy, you have to add it's shell configuration to your shell. ([learn why](https://lacy.tiimo.space/faq.html#why-does-it-need-a-shell-script))

<details>
<summary>ZSH</summary>

```shell
# ~/.zshrc
eval "$(lacy init zsh)"
```

</details>

<details>
<summary>Bash</summary>

```shell
# ~/.bashrc
eval "$(lacy init bash)"
```

</details>

<details>
<summary>Fish</summary>

```shell
# ~/.config/fish/config.fish
lacy init fish | source
```

</details>

<sup>If your shell is missing, feel free to create an issue!</sup>

---

## Contributions

Feel free to open a PR for any type of changes!

## Note

The code in this repository was written by hand. AI was used for the different shell configurations.

---

Built with <3 by [timothebot](https://github.com/timothebot)

<br>

*Haven't found what you are looking for? try looking in the [docs](https://lacy.tiimo.space/) :)*
