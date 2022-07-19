use std::path::PathBuf;

use eframe::egui::{self, Window, RichText, Layout, Button, Separator, Slider};
use mpdrs::Client;
use notify_rust::{Notification, Timeout};

use super::{Tangello, heading2, TangelloConfig, body2, configure_fonts};

impl Tangello {
    pub fn render_settings(&mut self, ctx: &egui::Context) -> bool {
        Window::new("Settings")
            .collapsible(false)
            .resizable(false)
            .title_bar(false)
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.label(RichText::new("Settings").text_style(heading2()));
                    ui.with_layout(Layout::right_to_left(), |ui| {
                        let close_settings_btn = ui.add(Button::new("❌"));
                        // When the close button is clicked store the settings to confy so they persist.
                        if close_settings_btn.clicked() {
                            if let Err(e) = confy::store(
                                "tangello",
                                TangelloConfig {
                                    dark_mode: self.config.dark_mode,
                                    music_path: self.config.music_path.clone(),
                                    tmp_music_path: self.config.tmp_music_path.clone(),
                                    mpd_address: self.config.mpd_address.to_string(),
                                    tmp_address: self.config.mpd_address.to_string(),
                                    notifications: self.config.notifications,
                                    scale: self.config.scale,
                                },
                            ) {
                                tracing::error!("Failed to save appstate: {}", e);
                            }
                            // Also close the settings window.
                            self.tmp_data.settings_open = false;
                        }
                    });
                });

                ui.add(Separator::default());
                ui.label(RichText::new("Enter your mpd ip address.").text_style(body2()));

                ui.horizontal(|ui| {
                    // Display the tmp address so the changes arn't reflected immediately (causes the program to crash if done in real time as it tries updating the address)
                    ui.text_edit_singleline(&mut self.config.tmp_address);
                    // When the apply button is pressed then set the address
                    if ui
                        .add(Button::new(RichText::new("Apply").text_style(body2())))
                        .clicked()
                    {
                        // Test that there is an mpd server at the address.
                        match Client::connect(self.config.tmp_address.clone()) {
                            Ok(_) => self.config.mpd_address = self.config.tmp_address.clone(),
                            Err(_) => {
                                match Notification::new()
                                    .summary("Tangello Music")
                                    .body("No mpd server found at that address.")
                                    .timeout(Timeout::Milliseconds(3500))
                                    .show()
                                {
                                    Err(_) => tracing::error!("No notification daemon active"),
                                    Ok(_) => (),
                                };
                            }
                        }
                    }
                });
                // Same thing as address minus the error notif and checking.
                ui.label(RichText::new("Enter your music folder.").text_style(body2()));

                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.config.tmp_music_path);
                    if ui
                        .add(Button::new(RichText::new("Apply").text_style(body2())))
                        .clicked()
                    {
                        self.config.music_path = PathBuf::from(self.config.tmp_music_path.clone());
                    }
                });

                egui::menu::bar(ui, |ui| {
                    // Toggle Darkmode.
                    ui.label(RichText::new("Dark Mode").text_style(body2()));
                    if ui
                        .add(Button::new({
                            if self.config.dark_mode {
                                ""
                            } else {
                                ""
                            }
                        }))
                        .clicked()
                    {
                        self.config.dark_mode = !self.config.dark_mode;
                    };
                });
                egui::menu::bar(ui, |ui| {
                    // Toggle notifications.
                    ui.label(RichText::new("Notifications").text_style(body2()));
                    if ui
                        .add(Button::new({
                            if self.config.notifications {
                                ""
                            } else {
                                ""
                            }
                        }))
                        .clicked()
                    {
                        self.config.notifications = !self.config.notifications;
                    };
                });
                    ui.label(RichText::new("Ui Scale.").text_style(body2()));
                    let scale_slider = ui.add(Slider::new(&mut self.config.scale, 0.5..=3.0));
                    if scale_slider.drag_released() {
                        configure_fonts(&self.config, ctx)
                    }
            });
        // Return that the settings are open
        true
    }
}