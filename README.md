# Lacy

<div align="center">
<img src="docs/assets/lacy_banner.png" width="700" alt="lacy banner">

<div>
    <a href="https://crates.io/crates/lacy"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/lacy"></a>
    <a href="https://crates.io/crates/lacy"><img alt="Crates.io Total Downloads" src="https://img.shields.io/crates/d/lacy"></a>
    <a href="https://github.com/timothebot/lacy/"><img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/timothebot/lacy"></a>
</div>
<br>
Lacy is a <i>magical cd alternative</i> that makes navigating much more efficient!
<br>
<br>
<sup>If you are interested, please consider ⭐-ing this repo so I know that more people are interested!</sup>
<br>
<br>
<img style="border-radius: 10px;" src="docs/assets/lacy_example_1.webp" width="500" alt="lacy example video">
<br>

</div>

## Features

### Navigation without full path

You don't have to type the full directory name.
Just make sure the path matches the directory name more or less.

```sh
# normal cd
cd /Users/timothebot/projects/lacy/src

# with lacy
y / user timo prj lacy sr
# or
y / usrs timbo rojt layc rc
```

### Skipping directories

Forgot the directory name? No problem. Just skip it using `-`.

```sh
y / user timo - lacy sr

# as long as some parts are still unique, it will work
# ...and it's very fast
y / - - - lacy sr
```

### Like cd

Everything you can do with `cd`, you can do with `y`.

```sh
y /Users/timo/Projects/Lacy/src
y ~
y ..
```

### Real paths

You can also add real paths while lazy navigating.

```sh
y /Users/timo/ desk proj lacy/src
```

## Getting started

### Installation

#### Cargo (Universal)

```sh
cargo install lacy
```

#### Homebrew (macOS)

```sh
brew install timothebot/tap/lacy
```

### Shell Setup

#### Zsh

```bash
# ~/.zshrc
eval "$(lacy init zsh)"
```

#### Bash

```bash
# ~/.bashrc
eval "$(lacy init bash)"
```

#### Fish

```bash
# ~/.config/fish/config.fish
lacy init fish | source
```

#### Other shells

Feel free to contribute the init script for your preferred shell.

## FAQ

### Why not `z`?

`z` is a great tool, but if you are working with a lot of projects with similar paths, it can be a pain to navigate. `z` also needs to learn first, while lacy can be used on systems you (or lacy) never touched before.
You can use `z` alongside lacy.

## Contributions

Feel free to open a PR for any type of changes!

## AI notice

The code in this repository was written by hand. AI was used for the different shell configurations and helped with the Github actions.

---

Built with <3 by [timothebot](https://github.com/timothebot)
