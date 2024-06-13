use crate::project::{Feature, Language, Project};

pub fn init() -> Project {
    cliclack::intro("Yorch's Development Toolbox").expect("Failed to print intro");
    let name: String = cliclack::input("Enter project name:")
        .placeholder("my-project")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Project name cannot be empty")
            } else {
                Ok(())
            }
        })
        .interact()
        .expect("Failed to get project name");

    let language = cliclack::select("Select a language:")
        .item(Language::Rust, "Rust", "")
        .item(Language::Go, "Go", "")
        .item(Language::Nodejs, "Node.js", "")
        .item(Language::None, "None", "Empty project")
        .interact()
        .expect("Failed to get language");

    let features = cliclack::multiselect("Select additional features:")
        .item(Feature::Git, "Git", "")
        .initial_values(vec![Feature::Git])
        .required(false)
        .interact()
        .expect("Failed to get additional features");

    cliclack::outro("Project created successfully!").expect("Failed to print outro");

    Project::new(name, language, features)
}
