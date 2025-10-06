# START generated Lacy shell config
{{ lacy_cmd }}() {
    new_path=$(lacy prompt {{ return_all }}-- "$*")
    if [ "$new_path" = "~" ]; then
        {{ cd }} ~
    {% if custom_fuzzy.enabled %}elif [[ "$new_path" == *$'\n'* ]]; then
        selected=$(printf "%s\n" "$new_path" | {{ custom_fuzzy.cmd }})
        [ -n "$selected" ] && {{ cd }} "$selected"
    {% endif %}elif [ -d "$new_path" ]; then
        {{ cd }} "$new_path" || return
    else
        echo "Error: No matching directory found for '$*'"
    fi
}
_{{ lacy_cmd }}() {
    local cur dirs
    cur="${COMP_WORDS[*]:1}"
    dirs=$(lacy complete --basename -- "$cur")
    COMPREPLY=($(compgen -W "$dirs" -- "$cur"))
}
_lacy() {
    local cur
    cur="${COMP_WORDS[COMP_CWORD]}"
    COMPREPLY=($(compgen -W "prompt complete init help" -- "$cur"))
}
complete -F _lacy -o default -o nospace lacy
complete -F _{{ lacy_cmd }} {{ lacy_cmd }}
# END generated Lacy shell config