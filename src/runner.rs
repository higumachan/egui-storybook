use crate::Story;
use askama::Template;
use std::path::Path;
use std::process::{Child, Command};
use std::sync::Arc;

pub enum State {
    Running(Child),
    Stopped,
}

pub struct StoryRunner {
    pub story: Arc<Story>,
    pub state: State,
    pub restart_to_changed: bool,
}

impl StoryRunner {
    pub fn run(&mut self, target_name: &str) -> anyhow::Result<()> {
        let workspace_dir = Path::new("target").join("egui_storybook");
        let crate_dir = workspace_dir.join("crates").join(&self.story.name);
        std::fs::create_dir_all(&crate_dir).unwrap();
        let cargo_toml = CargoToml {
            name: self.story.name.to_string(),
            target_crate_name: target_name.to_string(),
        }
        .render()
        .unwrap();
        let main_rs = Main {
            name: self.story.name.to_string(),
            main_code: self.story.body.to_string(),
        }
        .render()
        .unwrap();
        let workspace_toml_path = workspace_dir.join("Cargo.toml");
        let cargo_toml_path = crate_dir.join("Cargo.toml");
        let main_rs_path = crate_dir.join("src/main.rs");
        std::fs::create_dir_all(main_rs_path.parent().unwrap()).unwrap();
        std::fs::write(
            workspace_toml_path,
            WorkspaceCargoToml {
                target_crate_name: target_name.to_string(),
            }
            .render()
            .unwrap(),
        )
        .unwrap();
        std::fs::write(cargo_toml_path, cargo_toml).unwrap();
        std::fs::write(main_rs_path, main_rs).unwrap();

        self.state = State::Running(
            Command::new("cargo")
                .arg("run")
                .current_dir(crate_dir)
                .spawn()?,
        );

        Ok(())
    }

    pub fn stop(&mut self) {
        if let State::Running(child) = &mut self.state {
            child.kill().unwrap();
        }
        self.state = State::Stopped;
    }

    pub fn is_running(&self) -> bool {
        matches!(self.state, State::Running(_))
    }

    pub fn is_stopped(&self) -> bool {
        matches!(self.state, State::Stopped)
    }
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
    target_crate_name: String,
}

#[derive(Template)]
#[template(path = "WorkspaceCargo.toml.txt")]
pub struct WorkspaceCargoToml {
    target_crate_name: String,
}
