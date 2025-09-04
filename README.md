# Lacy

<div align="center">
<img src="docs/assets/lacy_banner.png" width="900" alt="lacy banner">

<div>
    <img alt="Crates.io Version" src="https://img.shields.io/crates/v/lacy">
    <img alt="Crates.io Total Downloads" src="https://img.shields.io/crates/d/lacy">
    <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/timothebot/lacy">
</div>
<br>
Lacy is a magical cd alternative that makes navigating much more efficient!

<div>
    <sup>If you are interested, please consider ⭐-ing this repo so I know that more people are interested!</sup>
</div>
<br>

<img src="docs/assets/example_0.webp" width="900" alt="lacy example video">

</div>

## Getting started

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

# as long as some parts are still unique, it will work... and it's very fast
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

## Installation

```sh
# macOS or Linux
cargo install lacy
```

### Zsh

Add the following to your `.zshrc` file:

```shell
eval "$(lacy init zsh)"
```

### Other shells

Feel free to contribute the init script for your shell. I'm gonna add them as soon as I can.

## Why not `z`?

`z` is a great tool, but if you are working with a lot of projects with similar paths, it can be a pain to navigate. `z` also needs to learn first, while lacy can be used on systems you (or lacy) never touched before.
You can use `z` alongside lacy.

---

Built with <3 by timothebot
