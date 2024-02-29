use eframe::NativeOptions;
use egui_storybook::story_book::{Story, StoryBookBuilder};
use egui_storybook::{run_story_book, story_body};

fn main() {
    let story_book = StoryBookBuilder::new()
        .add_story(Story::new(
            "hello_world",
            story_body! {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label(format!("Hello, world!"));
                });
            },
            vec![],
        ))
        .add_story(Story::new(
            "button_with_context_menu",
            story_body! {
                egui::CentralPanel::default().show(ctx, |ui| {
                    demo_project::button_with_context_menu(ui);
                });
            },
            vec![],
        ))
        .build();

    run_story_book("demo_project", story_book, NativeOptions::default()).unwrap();
}
