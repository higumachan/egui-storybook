use std::path::Path;
use askama::Template;
use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use tempdir::TempDir;

pub struct Story {
    pub name: String,
    pub body: TokenStream,
}

#[derive(Template)]
#[template(path = "main.txt")]
pub struct Main {
    name: String,
    main_code: String,
}

#[derive(Template)]
#[template(path = "cargo.toml.txt")]
pub struct CargoToml {
    name: String,
}


impl Story {
    pub fn run(self) {
        let cache_dir = Path::new("target").join("egui_storybook").join(&self.name);
        std::fs::create_dir_all(&cache_dir).unwrap();
        let cargo_toml = CargoToml { name: self.name.to_string() }.render().unwrap();
        let main_rs = Main { name: self.name.to_string(), main_code: self.body.to_string() }.render().unwrap();
        let cargo_toml_path = cache_dir.join("Cargo.toml");
        let main_rs_path = cache_dir.join("src/main.rs");
        std::fs::create_dir_all(main_rs_path.parent().unwrap()).unwrap();
        std::fs::write(cargo_toml_path, cargo_toml).unwrap();
        std::fs::write(main_rs_path, main_rs).unwrap();
        let status = std::process::Command::new("cargo")
            .current_dir(&cache_dir)
            .arg("run")
            .status()
            .unwrap();
    }
}