# Lacy 💤

Lacy is a z inspired cd alternative. It aims to make navigating easier for lazy people.

## Examples

![Example 0](docs/assets/example_0.webp)

```sh
# normal cd
$ cd /Users/timothebot/Desktop/projects/lacy/src

# with lacy
$ y / user timo desk proj lacy sr
```

## Installation

### Zsh

Add the following to your `.zshrc` file:

```shell
eval "$(lacy init zsh)"
```

## Why not `z`?

`z` is a great tool, but if you are working with a lot of projects with similar paths, it can be a pain to navigate.
You can use `z` alongside lacy.

## Todo

-   [x] Add support for `..`
-   [x] Add support for `~`
-   [x] Add support for "real" paths (e.g. `/Users/timo/ desk proj lacy`)
-   [x] Add ability to skip a directory
-   [ ] Add tests
-   [ ] Add aliases
