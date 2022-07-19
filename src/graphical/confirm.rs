use eframe::{egui::{self, Window, Layout}, emath};

use super::Tangello;

impl Tangello {
    pub fn confirm_window(&mut self, title: String, ctx: &egui::Context) -> bool {
        let mut result = false;
        Window::new(title)
            .collapsible(false)
            .resizable(false)
            .fixed_size(emath::Vec2 { x: (95. * self.config.scale), y: (50. * self.config.scale) })
            .show(ctx, |ui|{

            ui.with_layout(Layout::left_to_right(), |ui|{
                if ui.button("Yes").clicked() {
                    self.tmp_data.confirm_open = false;
                    result = true;
                } 
                ui.add_space(115. * self.config.scale);
                if ui.button("No").clicked() {
                    self.tmp_data.confirm_open = false;
                }
            });
        });
        result
    }
}