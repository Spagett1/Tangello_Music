use std::collections::HashMap;

use eframe::egui::{self, Button, RichText, Separator, ScrollArea, Layout, CentralPanel, Ui, TextEdit};
use mpdrs::Client;
use super::{Tangello, PADDING, WHITE, heading2, body2, BLUE};

impl Tangello {
    pub fn playlist_add(&mut self, conn: &mut Client, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.with_layout(Layout::left_to_right(), |ui|{
                    if ui.add(Button::new(" Save")).clicked() {
                        for i in &self.tmp_data.add_playlist_songs {
                            match conn.pl_push(&self.tmp_data.new_playlist_name, &i.file) { _ => ()};
                        }
                    }
                });
                ui.add_space(Ui::available_width(ui) / 2. - 120. * self.config.scale);
                ui.label(RichText::new("Add To Playlist").text_style(heading2()));

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
                            let button = ui.add(Button::new(RichText::new("▶").color(WHITE).text_style(body2()))
                                    .frame(false).small());
                            if button.clicked() && !self.tmp_data.add_playlist_songs.contains(song)
                            {
                                self.tmp_data.add_playlist_songs.push(song.clone())
                            } else {
                                // let index = self.tmp_data.add_playlist_songs.clone().into_iter().position(|x| x == song.clone()).unwrap();
                                // println!("{}", index);
                            }

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