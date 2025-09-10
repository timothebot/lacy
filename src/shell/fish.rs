pub fn fish_config(
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
function {lacy_cmd}
    set new_path (lacy prompt -- {return_all}"$argv")
    if test "$new_path" = "~"
        {cd} ~
    {fuzzy_if}else if test -d "$new_path"
        {cd} "$new_path"
    else
        echo "Error: No matching directory found for '$argv'"
    end
end
function __{lacy_cmd}_autocomplete
    set args $argv
    if test "$args" = ""
        ls -D --icons=never -1
    else
        set dirs (string split ' ' (lacy complete -- "$args"))
        for dir in $dirs
            basename $dir
        end
    end
end
complete --no-files lacy -x -a "prompt complete init help"
complete --no-files {lacy_cmd} -r -a "(__{lacy_cmd}_autocomplete)"
# END generated Lacy shell config
"#,
        cd = cd_cmd,
        return_all = return_all,
        fuzzy_if = fuzzy_if,
        lacy_cmd = lacy_cmd
    )
}
