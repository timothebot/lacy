use dialoguer::{theme::ColorfulTheme, Select};

pub fn select(title: &str, options: Vec<&str>) -> String {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(title)
        .items(&options)
        .default(0)
        .interact()
        .unwrap();
    options[selection].to_string()
}
