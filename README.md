# Lacy 💤

Lacy is a z inspired cd alternative. It aims to make navigating easier for lazy people.

> Lacy is WIP! Please report any issues you encounter. Don't expect it to work perfectly yet.
> (I'm using it daily, it won't break/harm your system, but will sometimes throw an error... working on it)

## Examples

![Example 0](docs/assets/example_0.webp)

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

`z` is a great tool, but if you are working with a lot of projects with similar paths, it can be a pain to navigate.
You can use `z` alongside lacy.

## Todo

-   [x] Add support for `..`
-   [x] Add support for `~`
-   [x] Add support for "real" paths (e.g. `/Users/timo/ desk proj lacy`)
-   [x] Add ability to skip a directory
-   [ ] Add more tests
-   [ ] Fix a lot of bugs
-   [ ] Add aliases
-   [ ] Add git cliff
-   [ ] Add support for more shells
