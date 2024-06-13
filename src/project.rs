#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Language {
    Rust,
    Go,
    Nodejs,
    None,
}

impl Language {
    pub fn nix_packages(&self) -> Vec<&str> {
        match self {
            Self::Rust => vec!["rustc", "cargo"],
            Self::Go => vec!["go"],
            Self::Nodejs => vec!["nodejs"],
            Self::None => vec![],
        }
    }

    pub fn available_features(&self) -> Vec<FeaturePromptItem> {
        match self {
            Self::Rust => vec![Feature::Git.to_prompt_item()],
            Self::Go => vec![Feature::Git.to_prompt_item()],
            Self::Nodejs => vec![Feature::Git.to_prompt_item()],
            Self::None => vec![Feature::Git.to_prompt_item()],
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
}

impl Feature {
    pub fn to_prompt_item(&self) -> (Self, &str, &str) {
        match self {
            Self::Git => (Self::Git, "Git", ""),
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
}
