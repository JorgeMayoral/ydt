#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Language {
    Rust,
    Go,
    Nodejs,
    None,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Feature {
    Git,
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
