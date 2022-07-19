use eframe::egui::{TopBottomPanel, self, Layout, RichText, Button, Ui};
use super::Tangello;

impl Tangello {
    pub fn render_top_panel(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(7.);
            egui::menu::bar(ui, |ui| {
                ui.with_layout(Layout::left_to_right(), |ui| {
                let sidebar_btn =
                    ui.button(RichText::new("  ").text_style(egui::TextStyle::Heading));
                // Sets the sidebar_open value, this decides whether to open the sidebar or close it.
                if sidebar_btn.clicked() && !self.tmp_data.sidebar_open {
                    self.tmp_data.sidebar_open = true;
                } else if sidebar_btn.clicked() && self.tmp_data.sidebar_open {
                    self.tmp_data.sidebar_open = false;
                }

                });
                ui.add_space(Ui::available_width(ui) / 2. - 120. * self.config.scale);
                // ui.horizontal_centered(|ui|{
                ui.heading("Tangello Music");
                // });

                ui.with_layout(Layout::right_to_left(), |ui| {
                    // Adds a close button and quits the program when pressed
                    if ui.add(Button::new("❌")).clicked() {
                        frame.quit();
                    }

                    // Sets the settings open value to true
                    // if ui.add(Button::new("漣")).clicked() {
                    //     self.tmp_data.settings_open = Tangello::render_settings(&mut self, ctx)
                    // } else if self.tmp_data.settings_open {
                    //     Tangello::render_settings(&mut self, ctx);
                    // }
                    if ui.add(Button::new("漣")).clicked() {
                        self.tmp_data.settings_open = true
                    } 
                })
            });
            ui.add_space(10.);
        });
    }
}