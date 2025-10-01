use upon::{value, Engine, Error};

use crate::cmd::{Init, Run};

impl Run for Init {
    fn run(&self) {
        println!(
            "{}",
            match shell_config(
                &self.shell.as_str(),
                &self.cd_cmd,
                &self.cmd,
                &self.custom_fuzzy
            ) {
                Ok(config) => config,
                Err(err) => format!("An error occurred: {}", err),
            }
        );
    }
}

pub fn shell_config(
    shell: &str,
    cd_cmd: &String,
    cmd: &String,
    custom_fuzzy: &Option<String>,
) -> Result<String, Error> {
    let mut engine = Engine::new();

    let _ = engine.add_template("bash", include_str!("../../templates/bash.sh"));
    let _ = engine.add_template("zsh", include_str!("../../templates/zsh.sh"));
    let _ = engine.add_template("fish", include_str!("../../templates/fish.fish"));

    engine
        .template(shell)
        .render(value! {
            cd: cd_cmd,
            lacy_cmd: cmd,
            return_all: if custom_fuzzy.is_some() {
                String::from("--return-all ")
            } else {
                String::new()
            },
            custom_fuzzy: {
                enabled: &custom_fuzzy.is_some(),
                cmd: custom_fuzzy.clone().unwrap_or_default()
            }
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_templates_compile() {
        let mut engine = upon::Engine::new();
        engine
            .add_template("bash", include_str!("../../templates/bash.sh"))
            .unwrap();
        engine
            .add_template("zsh", include_str!("../../templates/zsh.sh"))
            .unwrap();
        engine
            .add_template("fish", include_str!("../../templates/fish.fish"))
            .unwrap();
    }
}
