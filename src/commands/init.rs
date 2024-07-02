use crate::{
    language::Language,
    project::{create_dev_project, features_prompt, DevProject, Project},
};

pub fn init() {
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
        .item(Language::None, "None", "Empty project")
        .interact()
        .expect("Failed to get language");

    let features = features_prompt(&language);
    let project = create_dev_project(name, language, features);
    match project {
        Project::Rust(p) => p.create_project(),
        Project::Go(p) => p.create_project(),
        Project::Empty(p) => p.create_project(),
    }

    cliclack::outro("Project created successfully!").expect("Failed to print outro");
}
