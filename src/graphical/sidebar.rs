use eframe::{egui::{self, SidePanel, Ui, Button, RichText, Separator, ScrollArea, Window, Layout, TextEdit}, emath};
use mpdrs::Client;

use super::{Tangello, View, PADDING, WHITE, heading2};

impl Tangello {
    pub fn render_sidebar(&mut self, conn: &mut Client, ctx: &egui::Context) -> bool {
        self.tmp_data.panel_size = SidePanel::left("left_panel")
            .resizable(false)
            .default_width(109.35177)
            .show(ctx, |ui| {
                let panel_width: f32 = Ui::available_width(ui);
                if ui.add(Button::new("蘿 Queue").frame(false)).clicked() {
                    self.tmp_data.view = View::Queue;
                    self.tmp_data.sidebar_open = false
                }
                if ui.add(Button::new(" Library").frame(false).wrap(false)).clicked() {
                    self.grab_lib_data(conn);
                    self.tmp_data.view = View::Library;
                    self.tmp_data.sidebar_open = false;
                }
                ui.add_space(PADDING);
                ui.label(RichText::new("Playlists").color(WHITE).text_style(heading2()));
                ui.add(Separator::default());
                ScrollArea::vertical().show(ui, |ui|{
                    if ui.add(Button::new("New Playlist").small()
                    .frame(false)).clicked() {
                        self.tmp_data.new_playlist_open = true;
                        self.grab_lib_data(conn)
                    }
                    if self.tmp_data.new_playlist_open {
                        Window::new("Create Playlist.")
                            .fixed_size(emath::Vec2 { x: (95. * self.config.scale), y: (60. * self.config.scale) })
                            .collapsible(false)
                            .resizable(false)
                            .show(ctx, |ui|{

                            ui.add_sized([230. * self.config.scale, 10. * self.config.scale], TextEdit::singleline(&mut self.tmp_data.new_playlist_name));
                            ui.with_layout(Layout::left_to_right(), |ui|{
                                if ui.button("Yes").clicked() {
                                    self.tmp_data.sidebar_open = false;
                                    self.tmp_data.new_playlist_open = false;
                                    self.tmp_data.view = View::AddToPlaylist;
                                } 
                                ui.add_space(130. * self.config.scale);
                                if ui.button("No").clicked() {
                                    self.tmp_data.new_playlist_open = false;
                                }
                            });
                        });
                    }
                    for i in conn.playlists().unwrap() {
                        ui.horizontal(|ui|{
                            let result = ui.add(Button::new(&i.name).small().frame(false));
                            if result.clicked() {
                                self.tmp_data.selected_playlist.clear();
                                self.tmp_data.selected_playlist.push(i.clone());
                                self.tmp_data.view = View::Playlist;
                                self.tmp_data.sidebar_open = false;
                            }
                            ui.with_layout(Layout::right_to_left(), |ui|{
                                if ui.add(Button::new("").small().frame(false)).clicked() {
                                    self.tmp_data.confirm_open = true;
                                }
                                if self.tmp_data.confirm_open {
                                    if self.confirm_window("Delete Playlist".to_string(), ctx) {
                                        self.tmp_data.view = View::Queue;
                                        match conn.pl_remove(&i.name) { _ => () }
                                    }
                                }
                            });
                            
                        });
                    }
                });
                panel_width
            })
            .inner;
        true
    }
}