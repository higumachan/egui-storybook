use egui::{Response, Ui};

pub fn button_with_context_menu(ui: &mut Ui) -> Response {
    let response = ui.button("Click me");

    response.context_menu(|ui| {
        if ui.button("Do something").clicked() {
            // Do something
        }
    });

    response
}
