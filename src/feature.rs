pub type Features = Vec<Feature>;
type FeaturePromptItem = (Feature, &'static str, &'static str);
pub type AvailableFeatures = Vec<FeaturePromptItem>;
pub type DefaultFeatures = Features;
pub type ProjecFeatures = (AvailableFeatures, DefaultFeatures);

#[derive(Clone, PartialEq, Eq, Debug, Copy)]
pub enum Feature {
    Git,
    Nix,
    Air,
}

impl Feature {
    pub fn to_prompt_item(&self) -> (Self, &str, &str) {
        match self {
            Self::Git => (Self::Git, "Git", ""),
            Self::Nix => (Self::Nix, "Nix", ""),
            Self::Air => (Self::Air, "Air", "Live reload for Go apps"),
        }
    }
}
