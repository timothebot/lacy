
pub fn zsh_config(
    cd_cmd: String,
    return_all: String,
    custom_fuzzy: Option<String>,
    lacy_cmd: String,
) -> String {
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
function {lacy_cmd} {{
    new_path=$(lacy prompt -- {return_all}"$*")
    if [ "$new_path" = "~" ]; then
        {cd} ~
    {fuzzy_if}elif [ -d "$new_path" ]; then
        {cd} "$new_path"
    else
        echo "Error: No matching directory found for '$*'"
    fi
}}
function _{lacy_cmd} {{
    local dirs
    args="${{words[@]:1}}"
    dirs=$(lacy complete -- "$args")
    dirs=(${{(s: :)dirs}})
    compadd $dirs
}}
_lacy() {{
    compadd prompt complete init help
}}
compdef _lacy lacy
compdef _{lacy_cmd} {lacy_cmd}
# END generated Lacy shell config
"#,
        cd = cd_cmd,
        return_all = return_all,
        fuzzy_if = fuzzy_if,
        lacy_cmd = lacy_cmd
    )
}
