use empty::EmptyProject;
use go::GoProject;
use rust::RustProject;

use crate::{
    feature::{Features, ProjecFeatures},
    files::nix::NixFlake,
    language::Language,
};

mod empty;
mod go;
mod rust;

pub enum Project {
    Rust(RustProject),
    Go(GoProject),
    Empty(EmptyProject),
}

pub fn create_dev_project(name: String, lang: Language, features: Features) -> Project {
    match lang {
        Language::Rust => Project::Rust(RustProject::new(name, features)),
        Language::Go => Project::Go(GoProject::new(name, features)),
        Language::None => Project::Empty(EmptyProject::new(name, features)),
    }
}

pub fn features_prompt(lang: &Language) -> Features {
    match lang {
        Language::Rust => RustProject::features_prompt(),
        Language::Go => GoProject::features_prompt(),
        Language::None => EmptyProject::features_prompt(),
    }
}

pub trait DevProject {
    fn new(name: String, features: Features) -> Self;
    fn project_features() -> ProjecFeatures;
    fn features_prompt() -> Features {
        let (available_features, default_features) = Self::project_features();

        cliclack::multiselect("Select additional features:")
            .items(&available_features)
            .initial_values(default_features)
            .required(false)
            .interact()
            .expect("Failed to get additional features")
    }
    fn nix_packages(&self) -> Vec<String>;
    fn create_project(&self);
    fn create_nix_flake(&self) {
        let nix_packages = self.nix_packages();
        let nix_flake = NixFlake::new(nix_packages.to_owned());
        let flake_content = nix_flake.create();
        std::fs::write("./flake.nix", flake_content).expect("Failed to write file")
    }
}
