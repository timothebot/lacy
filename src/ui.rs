use dialoguer::{theme::ColorfulTheme, Select};

pub fn select(title: &str, options: Vec<String>) -> Option<String> {
    if let Some(selection) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(title)
        .items(&options)
        .default(0)
        .interact_opt()
        .ok()
        .flatten()
    {
        return Some(options[selection].to_string());
    }
    None
}
