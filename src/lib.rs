pub mod runner;
pub mod story_book;

use crate::runner::{State, StoryRunner};
use crate::story_book::{Story, StoryBook};
use askama::Template;
use eframe::emath::Align;
use eframe::NativeOptions;
use egui::{CentralPanel, Layout, SidePanel};
use quote::TokenStreamExt;
use std::collections::HashMap;
use std::sync::Arc;

pub fn run_story_book(
    target_name: &str,
    story_book: StoryBook,
    options: NativeOptions,
) -> eframe::Result<()> {
    let mut state = StoryBookState::new(story_book, target_name.to_string());

    eframe::run_simple_native("Storybook", options, move |ctx, _frame| {
        SidePanel::left("stories").show(ctx, |ui| state.left_panel(ui));
        CentralPanel::default().show(ctx, |ui| state.main_panel(ui));
    })
}

pub struct StoryBookState {
    pub book: StoryBook,
    pub current_story: Option<Arc<Story>>,
    runners: HashMap<String, StoryRunner>,
    target_name: String,
}

impl StoryBookState {
    pub fn new(book: StoryBook, target_name: String) -> Self {
        Self {
            book,
            target_name,
            current_story: None,
            runners: HashMap::new(),
        }
    }

    pub fn left_panel(&mut self, ui: &mut egui::Ui) {
        for story in &self.book.stories {
            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                ui.label(story.name.clone());

                let runner = self.runners.get_mut(&story.name);
                match runner {
                    Some(runner) if runner.is_running() => {
                        if ui.button("⏹").clicked() {
                            runner.stop();
                        }
                    }
                    _ => {
                        if ui.button("▶").clicked() {
                            let mut runner = StoryRunner {
                                story: story.clone(),
                                state: State::Stopped,
                                restart_to_changed: false,
                            };
                            runner.run(self.target_name.as_str()).unwrap();
                            self.runners.insert(story.name.clone(), runner);
                        }
                    }
                }
            });
        }
    }

    pub fn main_panel(&mut self, ui: &mut egui::Ui) {
        if let Some(story) = &self.current_story {
            ui.label(story.name.clone());
            ui.separator();
            ui.code(story.body.to_string());
        }
    }
}
