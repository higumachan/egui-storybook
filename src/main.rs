use quote::quote;
use egui_storybook::Story;

fn main() {
    let a = 10;
    let token = quote! {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("Hello, world!"));
        });
    };

    let story = Story {
        name: "hello_world".to_string(),
        body: token,
    };

    story.run();

}
