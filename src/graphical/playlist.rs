use std::collections::HashMap;

use eframe::egui::{self, Button, RichText, Separator, ScrollArea, Layout, CentralPanel};
use mpdrs::Client;

use super::{Tangello, PADDING, WHITE, heading2, body2, BLUE};
impl Tangello {
    pub fn render_playlist(&mut self, conn: &mut Client, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            let current_playlist= self.tmp_data.selected_playlist[0].clone();
            egui::menu::bar(ui, |ui|{
                ui.vertical_centered(|ui|{
                    ui.label(RichText::new(&current_playlist.name).text_style(heading2()));
                });
                ui.with_layout(Layout::right_to_left(), |ui| {
                    ui.add_space(20.);
                    if ui.add(Button::new("")).clicked() {
                        self.tmp_data.confirm_open = true;
                    }
                    if self.tmp_data.confirm_open && self.confirm_window("Clear Playlist?".to_string(), ctx) &&
                    conn.pl_clear(&current_playlist.name).is_err() {
                        tracing::error!("Could not clear playlist.")
                    }
                });
            });

            ui.add_space(3.5);
            ui.separator();
            ScrollArea::vertical()
                .max_height(ui.available_height() - 63.)
                .show(ui, |ui| {

                    // let mut pos = 0;
                    // for song in conn.playlist(current_playlist.name.as_str()).unwrap() 
                    for (pos, song) in conn.playlist(current_playlist.name.as_str()).unwrap().into_iter().enumerate() {
                        // pos += 1;
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
                            if ui.add(Button::new("羅").frame(false)).clicked() {
                                let position = if (pos - 1) <  4294967295 {
                                    tracing::error!("How on earth do you have more than 4294967295 songs?"); 
                                    4294967295
                                } else {
                                    (pos - 1).try_into().unwrap()
                                };
                                match conn.pl_delete(&current_playlist.name, position ) {
                                    Ok(_) => (),
                                    Err(_) => tracing::error!("Song does not exist."),
                                }
                            }
                            ui.with_layout(Layout::right_to_left(), |ui| {
                                ui.add(
                                    Button::new(RichText::new(album).color(BLUE))
                                        .frame(false)
                                        .small(),
                                );
                            });
                        });
                        ui.add(Separator::default());
                    }
                    ui.add_space(40.);
                });
        });

    }
}