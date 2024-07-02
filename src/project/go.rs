use crate::{
    feature::{Feature, Features},
    project::DevProject,
};

pub struct GoProject {
    name: String,
    features: Features,
}

impl GoProject {
    fn create_dir(&self) {
        let dir_path = format!("./{}", self.name);
        std::fs::create_dir(dir_path).expect("Couldn't create project directory.");
    }

    fn initialize_git(&self) {
        std::process::Command::new("git")
            .arg("init")
            .status()
            .expect("Failed launching \"git init\" command.");
    }

    fn initialize_project(&self) {
        let project_domain: String = cliclack::input("Enter project domain:")
            .placeholder("name.example.com")
            .validate(|input: &String| {
                if input.is_empty() {
                    Err("Project name cannot be empty")
                } else {
                    Ok(())
                }
            })
            .interact()
            .expect("Failed to get project domain");
        std::process::Command::new("go")
            .args(["mod", "init", &project_domain])
            .status()
            .expect("Failed launching \"go mod init\" command.");
    }
}

impl DevProject for GoProject {
    fn new(name: String, features: Features) -> Self {
        GoProject { name, features }
    }

    fn project_features() -> crate::feature::ProjecFeatures {
        let available_features = vec![
            Feature::Git.to_prompt_item(),
            Feature::Nix.to_prompt_item(),
            Feature::Air.to_prompt_item(),
        ];
        let default_features = vec![Feature::Git];
        (available_features, default_features)
    }

    fn nix_packages(&self) -> Vec<String> {
        let mut nix_pkgs = vec!["go".to_owned(), "gopls".to_owned()];
        if self.features.contains(&Feature::Air) {
            nix_pkgs.push("air".to_owned());
        }
        nix_pkgs
    }

    fn create_project(&self) {
        self.create_dir();
        std::env::set_current_dir(&self.name).unwrap();
        self.initialize_project();
        if self.features.contains(&Feature::Git) {
            self.initialize_git();
        }
        if self.features.contains(&Feature::Nix) {
            self.create_nix_flake();
        }
    }
}
