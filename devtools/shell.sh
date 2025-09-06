# source this with zsh to use dev/release build

function yr {
    new_path=$(../target/release/lacy prompt -- "$*")
    if [ "$new_path" = "~" ]; then
        cd ~
    elif [ -d "$new_path" ]; then
        cd "$new_path"
    else
        echo "Error: No matching directory found for '$*'"
    fi
}
function _yr {
    local dirs
    args="${{words[@]:1}}"
    dirs=$(../target/release/lacy complete -- "$args")
    dirs=(${{(s: :)dirs}})
    compadd $dirs
}
compdef _yr yr


function yd {
    new_path=$(../target/debug/lacy prompt -- "$*")
    if [ "$new_path" = "~" ]; then
        cd ~
    elif [ -d "$new_path" ]; then
        cd "$new_path"
    else
        echo "Error: No matching directory found for '$*'"
    fi
}
function _yd {
    local dirs
    args="${{words[@]:1}}"
    dirs=$(../target/debug/lacy complete -- "$args")
    dirs=(${{(s: :)dirs}})
    compadd $dirs
}
compdef _yd yd
