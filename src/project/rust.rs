use crate::{
    feature::{Feature, Features},
    project::DevProject,
};

pub struct RustProject {
    name: String,
    features: Features,
}

impl RustProject {
    fn initialize_project(&self) {
        let mut cargo_new_cmd = std::process::Command::new("cargo");
        cargo_new_cmd.args(["new", &self.name]);
        if !self.features.contains(&Feature::Git) {
            cargo_new_cmd.args(["--vcs", "none"]);
        }
        cargo_new_cmd
            .status()
            .expect("Failed launching \"cargo new\" command.");
    }
}

impl DevProject for RustProject {
    fn new(name: String, features: Features) -> Self {
        RustProject { name, features }
    }

    fn project_features() -> crate::feature::ProjecFeatures {
        let available_features = vec![Feature::Git.to_prompt_item(), Feature::Nix.to_prompt_item()];
        let default_features = vec![Feature::Git];
        (available_features, default_features)
    }

    fn nix_packages(&self) -> Vec<String> {
        vec!["rustc".to_owned(), "cargo".to_owned()]
    }

    fn create_project(&self) {
        self.initialize_project();
        std::env::set_current_dir(&self.name).unwrap();
        if self.features.contains(&Feature::Nix) {
            self.create_nix_flake();
        }
    }
}
