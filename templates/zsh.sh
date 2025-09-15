# START generated Lacy shell config
function {{ lacy_cmd }} {
    new_path=$(lacy prompt {{ return_all }}-- "$*")
    if [ "$new_path" = "~" ]; then
        {{ cd }} ~
    {% if custom_fuzzy.enabled %}elif [[ "$new_path" == *$'\n'* ]]; then
        selected=$(printf "%s\n" "$new_path" | {{ custom_fuzzy.cmd }})
        [ -n "$selected" ] && {{ cd }} "$selected"
    {% endif %}elif [ -d "$new_path" ]; then
        {{ cd }} "$new_path"
    else
        echo "Error: No matching directory found for '$*'"
    fi
}
function _{{ lacy_cmd }} {
    local dirs
    args="${words[@]:1}"
    dirs=$(lacy complete -- "$args")
    dirs=(${(s: :)dirs})
    compadd $dirs
}
_lacy() {
    compadd prompt complete init help
}
compdef _lacy lacy
compdef _{{ lacy_cmd }} {{ lacy_cmd }}
# END generated Lacy shell config