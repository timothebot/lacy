use dialoguer::Select;

pub fn select(title: &str, options: Vec<&str>) -> String {
    let selection = Select::new()
        .with_prompt(title)
        .items(&options)
        .interact()
        .unwrap();
    options[selection].to_string()
}
