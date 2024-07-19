pub fn get_shell_config(shell: &str) {
    match shell {
        "zsh" | "bash" => {
            println!(
                r#"function y {{
    new_path=$(lacy prompt "$*")
    if [ -d "$new_path" ]; then
        cd "$new_path"
    else
        echo "Error: No matching directory found for '$*'"
    fi
}}"#
            );
        }
        _ => {
            eprintln!("Error: Unsupported shell '{}'", shell);
        }
    }
}
