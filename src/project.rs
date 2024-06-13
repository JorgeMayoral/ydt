use crate::files::nix::NixFlake;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Language {
    Rust,
    Go,
    Nodejs,
    None,
}

impl Language {
    pub fn nix_packages(&self) -> Vec<String> {
        match self {
            Self::Rust => vec!["rustc".to_owned(), "cargo".to_owned()],
            Self::Go => vec!["go".to_owned()],
            Self::Nodejs => vec!["nodejs".to_owned()],
            Self::None => vec![],
        }
    }

    pub fn available_features(&self) -> Vec<FeaturePromptItem> {
        match self {
            Self::Rust => vec![Feature::Git.to_prompt_item(), Feature::Nix.to_prompt_item()],
            Self::Go => vec![Feature::Git.to_prompt_item(), Feature::Nix.to_prompt_item()],
            Self::Nodejs => vec![Feature::Git.to_prompt_item(), Feature::Nix.to_prompt_item()],
            Self::None => vec![Feature::Git.to_prompt_item(), Feature::Nix.to_prompt_item()],
        }
    }

    pub fn default_features(&self) -> Vec<Feature> {
        match self {
            _ => vec![Feature::Git],
        }
    }
}

type FeaturePromptItem<'a> = (Feature, &'a str, &'a str);

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Feature {
    Git,
    Nix,
}

impl Feature {
    pub fn to_prompt_item(&self) -> (Self, &str, &str) {
        match self {
            Self::Git => (Self::Git, "Git", ""),
            Self::Nix => (Self::Nix, "Nix", ""),
        }
    }
}

#[derive(Debug)]
pub struct Project {
    name: String,
    language: Language,
    features: Vec<Feature>,
}

impl Project {
    pub fn new(name: String, language: Language, features: Vec<Feature>) -> Self {
        Self {
            name,
            language,
            features,
        }
    }

    pub fn handle_features(&self) {
        for feature in &self.features {
            match feature {
                Feature::Git => {
                    println!("Handling Git feature");
                }
                Feature::Nix => {
                    println!("Handling Nix feature");
                    let nix_packages = &self.language.nix_packages().clone();
                    let nix_flake = NixFlake::new(nix_packages.to_owned());
                    let flake_content = nix_flake.create();
                    Self::write_file("./flake.nix".to_string(), flake_content);
                }
            }
        }
    }

    fn write_file(path: String, content: String) {
        std::fs::write(path, content).expect("Failed to write file");
    }

    pub fn features(&self) -> &[Feature] {
        &self.features
    }
}
