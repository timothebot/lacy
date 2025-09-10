use crate::shell::{bash_config, fish_config, zsh_config};

pub fn shell_config(
    shell: &str,
    cd_cmd: Option<String>,
    custom_fuzzy: Option<String>,
    lacy_cmd: Option<String>,
) {
    let cd_cmd = cd_cmd.unwrap_or(String::from("cd"));
    let lacy_cmd = lacy_cmd.unwrap_or(String::from("y"));
    let return_all = if custom_fuzzy.is_some() {
        String::from("--return-all ")
    } else {
        String::new()
    };
    match shell {
        "zsh" => {
            println!("{}", zsh_config(cd_cmd, return_all, custom_fuzzy, lacy_cmd))
        }
        "bash" => {
            println!("{}", bash_config(cd_cmd, return_all, custom_fuzzy, lacy_cmd))
        }
        "fish" => {
            println!("{}", fish_config(cd_cmd, return_all, custom_fuzzy, lacy_cmd))
        }
        _ => {
            eprintln!("Error: Unsupported shell '{}'", shell);
        }
    }
}
