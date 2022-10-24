use std::collections::HashMap;

use eframe::egui::{self, Button, RichText, Separator, ScrollArea, Layout, CentralPanel, TextEdit};
use mpdrs::Client;

use super::{Tangello, PADDING, WHITE, heading2, body2, BLUE};
impl Tangello {
    // Renders the library, very similar to rendering the queue.
    pub fn render_library(&mut self, conn: &mut Client, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.vertical_centered(|ui|{
                    ui.label(RichText::new("Library").text_style(heading2()));
                });

                ui.with_layout(Layout::right_to_left(), |ui| {
                    if self.tmp_data.search_bar {
                        ui.add_space(20.);
                        if ui.add(Button::new("")).clicked() {
                            self.tmp_data.search.clear();
                            self.tmp_data.search_bar = false;
                        }
                        let response = ui.add_sized(
                            [70. * self.config.scale, 10. * self.config.scale],
                            TextEdit::singleline(&mut self.tmp_data.search)
                                .hint_text("Search")
                                .desired_width(-10.),
                        );

                        
                        // If the enter key is pressed search
                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            self.tmp_data.songs.clear();
                            for i in self.tmp_data.songlist_vec.iter() {
                                let map: HashMap<_, _> = i.tags.clone().into_iter().collect();
                                let album = format!("{} ⤴", map["Album"]);
                                if i.title
                                    .clone()
                                    .unwrap()
                                    .to_lowercase()
                                    .contains(&self.tmp_data.search.to_lowercase())
                                    || i.artist
                                        .clone()
                                        .unwrap()
                                        .to_lowercase()
                                        .contains(&self.tmp_data.search.to_string().to_lowercase())
                                    || album
                                        .to_lowercase()
                                        .contains(&self.tmp_data.search.to_string().to_lowercase())
                                {
                                    self.tmp_data.songs.push(i.clone());
                                }
                            }
                        }
                        if self.tmp_data.search_bar_want_focus {
                            response.request_focus()
                        }
                        if response.clicked_elsewhere() {
                            response.surrender_focus();
                            self.tmp_data.search_bar_want_focus = false
                        }


                    } else {
                        self.tmp_data.songs = self.tmp_data.songlist_vec.clone();
                        ui.add_space(20.);
                        if ui.add(Button::new("")).clicked() {
                            self.tmp_data.search_bar = true;
                            self.tmp_data.search_bar_want_focus = true;
                        }
                    }
                });
            });
            ui.add_space(3.5);
            ui.separator();
            ScrollArea::vertical()
                .max_height(ui.available_height() - 63.)
                .show(ui, |ui| {
                    for song in self.tmp_data.songs.clone().iter() {
                        let map: HashMap<_, _> = song.tags.clone().into_iter().collect();
                        let album = format!("{} ⤴", map["Album"]);

                        ui.add_space(PADDING);
                        ui.horizontal(|ui| {
                            if ui
                                .add(
                                    Button::new(
                                        RichText::new("▶").color(WHITE).text_style(body2()),
                                    )
                                    .frame(false)
                                    .small(),
                                )
                                .clicked()
                            {
                                match conn.add(&song.file) {
                                    Ok(_) => (),
                                    Err(_) => tracing::error!("Song does not exist."),
                                }
                                let number_of_songs = conn.queue().unwrap().len();
                                let new_song = conn.queue().unwrap()[number_of_songs - 1]
                                    .place
                                    .unwrap()
                                    .pos;
                                match conn.play_from_position(new_song) {
                                    Ok(_) => Tangello::change_image(self, conn),
                                    Err(_) => tracing::error!("Song does not exist."),
                                }
                            };
                            ui.label(
                                RichText::new(song.title.as_ref().unwrap())
                                    .color(WHITE)
                                    .text_style(body2()),
                            );
                        });

                        ui.label(song.artist.as_ref().unwrap());

                        ui.add_space(PADDING);

                        ui.horizontal(|ui| {
                            if ui.add(Button::new("螺").frame(false)).clicked() {
                                match conn.add(&song.file) {
                                    Ok(_) => (),
                                    Err(_) => tracing::error!("Song does not exist."),
                                }
                            }
                            ui.add_space(20. * self.config.scale);
                            ui.add(
                                Button::new(RichText::new(album).color(BLUE))
                                    .frame(false)
                                    .small(),
                            );
                        });
                        ui.add(Separator::default());
                    }
                    ui.add_space(40.);
                });
        });
    }
}