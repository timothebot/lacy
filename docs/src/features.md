# Features

- [Improved CD Features](#improved-cd-features)
  - [Navigation](#navigation)
  - [Back Navigation](#back-navigation)
  - [Path History Navigation](#path-history-navigation)
- [New Lacy Features](#new-lacy-features)
  - [Skipping Directories](#skipping-directories)
  - [Multiple Results](#multiple-results)

To use lacy, you use the `y` command. (You can easily change the default key, see [FAQ](./faq.md))

## Improved CD Features

### Navigation

Lacy is `cd` compatible, meaning everything you can do with `cd`, you can do it the exact same way in lacy:

```bash
cd Desktop/foo/bar
y Desktop/foo/bar

cd /Users/tiimo
y /Users/tiimo

cd ~
y ~
```

But where `cd` is very strict about what you type, lacy isn't:

```bash
cd Desktop/foo/bar
y dskt/fooo/bar

cd /Users/tiimo
y / usrs/timoo
```

You can even write spaces instead of slashes if you want:

```bash
cd Desktop/foo/bar
y dskt fooo bar

cd /Users/tiimo
y / usrs  timoo
```

Lacy uses a custom fuzzy matching algorithm, specifically crafted for lazily writing paths
so you always find what you want!

### Back Navigation

Navigating back using `..` is also a feature that is improved.

```bash
cd ../..
y ../..
```

Instead of writing `../..` for each level, you can just type `...` (Some shells already have this functionality built in).

```bash
cd ../../..
y ....
```

### Path History Navigation

> This feature is currently WIP!

```bash
cd -1
y -1
```

## New Lacy Features

### Skipping Directories

If you ever know your target directory, but forgot whats between it, you can just skip it using `-`.

```bash
cd Desktop/foo/bar
y - foo bar
```

You can do that as many times as you want.

```bash
cd Desktop/foo/bar
y - - - foo bar
```

### Multiple Results

By now you probably have wondered what happens if lacy can't decide what your target
folder is and finds multiple results.

In that case, lacy opens a multiselect window where you can select your target

```text
? Multiple possibilities found! ›
❯ /Users/tiimo/Desktop/projects/lacy/devtools
  /Users/tiimo/Desktop/projects/lacy/docs
  /Users/tiimo/Desktop/projects/lacy/result
  /Users/tiimo/Desktop/projects/lacy/src
```

If you want, you can [easily swap out the default select](./setup.md#shell-options) with your own solution,
like for example `fzf`.
