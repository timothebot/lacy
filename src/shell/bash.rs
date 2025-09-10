pub fn bash_config(cd_cmd: String, return_all: String, custom_fuzzy: Option<String>, lacy_cmd: String) -> String {
    let mut fuzzy_if = String::new();
    if let Some(fuzzy_cmd) = custom_fuzzy {
        fuzzy_if = format!(
            r#"elif [[ "$new_path" == *$'\n'* ]]; then
        selected=$(printf "%s\n" "$new_path" | {fuzzy_cmd})
        [ -n "$selected" ] && {cd} "$selected"
    "#,
            cd = cd_cmd,
            fuzzy_cmd = fuzzy_cmd
        )
    };

    format!(
        r#"
# START generated Lacy shell config
{lacy_cmd}() {{
    new_path=$(lacy prompt -- {return_all}"$*")
    if [ "$new_path" = "~" ]; then
        {cd} ~
    {fuzzy_if}elif [ -d "$new_path" ]; then
        {cd} "$new_path" || return
    else
        echo "Error: No matching directory found for '$*'"
    fi
}}
_{lacy_cmd}() {{
    local cur dirs basenames
    cur="${{COMP_WORDS[*]:1}}"
    dirs=$(lacy complete -- "$cur")
    basenames=$(printf '%s\n' $dirs | xargs -n1 basename)
    COMPREPLY=($(compgen -W "$basenames" -- "$cur"))
}}
_lacy() {{
    local cur
    cur="${{COMP_WORDS[COMP_CWORD]}}"
    COMPREPLY=($(compgen -W "prompt complete init help" -- "$cur"))
}}
complete -F _lacy -o default -o nospace lacy
complete -F _{lacy_cmd} {lacy_cmd}
# END generated Lacy shell config
"#,
        cd = cd_cmd,
        return_all = return_all,
        fuzzy_if = fuzzy_if,
        lacy_cmd = lacy_cmd
    )
}
