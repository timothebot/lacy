# source this with zsh to use dev build

alias lacy="$(pwd)/$(dirname "$0")/../target/debug/lacy"

eval "$(lacy init zsh)"
