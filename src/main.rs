use cliclack as cli;

#[derive(Clone, PartialEq, Eq, Debug)]
enum AvailableLanguages {
    Rust,
    Go,
    Nodejs,
    None,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum AdditionalFeatures {
    Git,
}

fn main() {
    cli::intro("Yorch's Development Toolbox").expect("Failed to print intro");
    let project_name: String = cli::input("Enter project name:")
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

    let language = cli::select("Select a language:")
        .item(AvailableLanguages::Rust, "Rust", "")
        .item(AvailableLanguages::Go, "Go", "")
        .item(AvailableLanguages::Nodejs, "Node.js", "")
        .item(AvailableLanguages::None, "None", "Empty project")
        .interact()
        .expect("Failed to get language");

    let additional_features = cli::multiselect("Select additional features:")
        .item(AdditionalFeatures::Git, "Git", "")
        .initial_values(vec![AdditionalFeatures::Git])
        .required(false)
        .interact()
        .expect("Failed to get additional features");

    cli::outro("Project created successfully!").expect("Failed to print outro");

    println!("{project_name:#?} - {language:#?} - {additional_features:#?}")
}
