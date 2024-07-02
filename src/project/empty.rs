use crate::{
    feature::{Feature, Features},
    project::DevProject,
};

pub struct EmptyProject {
    name: String,
    features: Features,
}

impl EmptyProject {
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
}

impl DevProject for EmptyProject {
    fn new(name: String, features: Features) -> Self {
        EmptyProject { name, features }
    }

    fn project_features() -> crate::feature::ProjecFeatures {
        let available_features = vec![Feature::Git.to_prompt_item(), Feature::Nix.to_prompt_item()];
        let default_features = vec![Feature::Git];
        (available_features, default_features)
    }

    fn nix_packages(&self) -> Vec<String> {
        vec![]
    }

    fn create_project(&self) {
        self.create_dir();
        std::env::set_current_dir(&self.name).unwrap();
        if self.features.contains(&Feature::Git) {
            self.initialize_git();
        }
        if self.features.contains(&Feature::Nix) {
            self.create_nix_flake();
        }
    }
}
