pub fn get_shell_config(shell: &str) {
    match shell {
        "zsh" => {
            println!(
                r#"
function y {{
    new_path=$(lacy prompt -- "$*")
    if [ -d "$new_path" ]; then
        cd "$new_path"
    else
        echo "Error: No matching directory found for '$*'"
    fi
}}
function _y {{
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
compdef _y y"#
            );
        }
        "bash" => {
            println!(
                r#"
y() {{
    new_path=$(lacy prompt -- "$*")
    if [ -d "$new_path" ]; then
        cd "$new_path" || return
    else
        echo "Error: No matching directory found for '$*'"
    fi
}}
_y() {{
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
complete -F _y y"#
            )
        }
        "fish" => {
            println!(r#"
function y
    set new_path (lacy prompt -- "$argv")
    if test -d "$new_path"
        cd "$new_path"
    else
        echo "Error: No matching directory found for '$argv'"
    end
end
function __y_autocomplete
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
complete --no-files y -r -a "(__y_autocomplete)""#)
        }
        _ => {
            eprintln!("Error: Unsupported shell '{}'", shell);
        }
    }
}
