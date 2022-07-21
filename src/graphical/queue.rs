use std::collections::HashMap;

use eframe::egui::{self, Button, RichText, Separator, ScrollArea, Layout, CentralPanel};
use mpdrs::Client;

use super::{Tangello, PADDING, WHITE, heading2, body2, BLUE};
impl Tangello {
    pub fn render_queue(&mut self, conn: &mut Client, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new("Queue").text_style(heading2()));
                });

                ui.with_layout(Layout::right_to_left(), |ui| {
                    ui.add_space(20.);
                    if ui.add(Button::new("")).clicked() && conn.clear().is_err() {
                        tracing::error!("Cannot clear the queue.")
                    };
                });
            });

            ui.add_space(3.5);
            ui.separator();
            ScrollArea::vertical()
                .max_height(ui.available_height() - 63.)
                .show(ui, |ui| {
                    for a in conn.queue().expect("There are no songs in queue").iter() {
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
                                match conn.play_from_position(a.place.unwrap().pos) {
                                    Err(_) => tracing::error!(
                                        "I have no clue when this will ever get called"
                                    ),
                                    Ok(_) => Tangello::song_change(self, conn),
                                }
                            };
                            ui.label(
                                RichText::new(a.title.as_ref().unwrap())
                                    .color(WHITE)
                                    .text_style(body2()),
                            );
                        });

                        ui.label(a.artist.as_ref().unwrap());

                        ui.add_space(PADDING);

                        ui.horizontal(|ui| {
                            if ui.add(Button::new("羅").frame(false)).clicked() {
                                match conn.deleteid(a.place.unwrap().pos) {
                                    Ok(_) => (),
                                    Err(_) => tracing::error!("Song does not exist."),
                                }
                            }
                            ui.with_layout(Layout::right_to_left(), |ui| {
                                let map: HashMap<_, _> = a.tags.clone().into_iter().collect();
                                let album = format!("{} ⤴", map["Album"]);
                                ui.add(Button::new(RichText::new(album).color(BLUE)).frame(false).small());
                            });
                        });
                        ui.add_space(PADDING);
                        ui.add(Separator::default());
                    }
                    ui.add_space(40.);
                });
        });
    }
}