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
        let mut hb = Handlebars::new();
        hb.register_template_file("flake", "templates/flake.nix")
            .expect("Failed to register template file");
        hb.render("flake", &self)
            .expect("Failed to render template")
    }
}
