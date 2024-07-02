use handlebars::Handlebars;
use serde::Serialize;

#[derive(Serialize)]
pub struct NixFlake {
    packages: Vec<String>,
}

impl NixFlake {
    pub fn new(packages: Vec<String>) -> Self {
        Self { packages }
    }

    pub fn create(&self) -> String {
        let hb = Handlebars::new();
        let template = include_str!("../../templates/flake.nix");
        hb.render_template(&template, &self)
            .expect("Failed to render template")
    }
}
