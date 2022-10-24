use eframe::{egui::{self, Button, RichText, Layout, TopBottomPanel, Ui}, emath, epaint::Color32};
use mpdrs::{Client, State};
use super::{Tangello, WHITE, body2, heading3};

const SLIDER_COLOUR: Color32 = Color32::from_rgb(70, 70, 70);

impl Tangello {
    pub fn render_footer(&mut self, ctx: &egui::Context, conn: &mut Client) {
        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.vertical(|ui| {
                    ui.with_layout(Layout::left_to_right(), |ui| {
                        let window_width: f32 = Ui::available_width(ui);

                        // Render Album art
                        ui.image(
                            self.tmp_data.image.texture_id(ctx),
                            emath::Vec2 { x: (95. * self.config.scale), y: (95. * self.config.scale) },
                        );

                        // Display song name and artist, if no song is playing say that.
                        match conn.status().expect("Can not get the mpd state.").state {
                            State::Stop => {
                                ui.vertical(|ui|{
                                    ui.label(RichText::new("Nothing Playing").color(WHITE)
                                        .text_style(body2()));
                                });
                            },
                            _ => {
                                // Probably a better way to do this but check if the song is the same as last cycle and if not refresh the image
                                if self.tmp_data.prev_song.is_empty() {
                                    self.tmp_data.prev_song.push(conn.currentsong().unwrap().unwrap());
                                }
                                if self.tmp_data.prev_song[0] != conn.currentsong().unwrap().unwrap() {
                                    self.song_change(conn);
                                    self.tmp_data.prev_song[0] = conn.currentsong().unwrap().unwrap();
                                }
                                ui.vertical(|ui|{
                                    ui.label(RichText::new(conn.currentsong()
                                        .unwrap().unwrap()
                                        .title.unwrap())
                                        .color(WHITE)
                                        .text_style(body2()),
                                    );
                                    ui.label(RichText::new(
                                        conn.currentsong().unwrap().unwrap().artist.unwrap(),
                                    ));
                                });
                            },
                        }
                        ui.horizontal(|ui|{

                            let firstbutton = Ui::available_width(ui) - window_width / 2. - 50. * self.config.scale;
                            ui.add_space(firstbutton);
                            ui.label(RichText::new("").text_style(heading3()));

                            // Button plays the previous song.
                            if ui.add(Button::new(RichText::new('玲').text_style(egui::TextStyle::Heading))
                                .frame(false)).clicked() {
                                conn.prev().unwrap_or(());
                                Tangello::song_change(self, conn);
                            }
                            if ui.add(Button::new(RichText::new(match conn.status().expect("Could not get the mpd state.").state {
                                State::Play => "",
                                _ => "",
                                }).text_style(egui::TextStyle::Heading))
                                .frame(false)).clicked() 
                            {
                                match conn.status().expect("Could not get the mpd state.").state {
                                    State::Play => {
                                        conn.pause(true).expect("The pause state could not be toggled.");
                                    },
                                    _ => {
                                        conn.play().expect("Song could not be played.");
                                        Tangello::change_image(self, conn)
                                    },
                                }
                            }
                            ui.add_space(1. * self.config.scale);

                            // Button plays the next song.
                            if ui.add(Button::new(RichText::new('怜').text_style(egui::TextStyle::Heading))
                                .frame(false)).clicked() {
                                conn.next().unwrap_or(());
                                Tangello::song_change(self, conn);
                            }                           

                  
                        })
                    });
                    let window_width: f32 = Ui::available_width(ui);
                    // Sets the slider colours and spacing
                    Ui::style_mut(ui).visuals.widgets.inactive.bg_fill = SLIDER_COLOUR;
                    Ui::style_mut(ui).spacing.slider_width = window_width;

                    // If there is no song playing just show a dummy slider.
                    if conn.currentsong().unwrap() == None {
                        let mut dummy_size: f32 = 0.0;
                        ui.add(
                            egui::widgets::Slider::new(&mut dummy_size, 0.0..=100.0)
                                .show_value(false),
                        );
                    } else {
                        // Repaints the slider, this is needed otherwise the slider only updates when the mouse is on it.
                        // if conn.status().expect("can not get the mpd state.").state == State::Play {
                            // ctx.request_repaint();
                        // }
                        // Grabs the song length and the current position of the song.
                        let mut current_place = conn.status().unwrap().elapsed.unwrap().as_secs_f64();
                        let re = ui.add(
                            egui::widgets::Slider::new(&mut current_place, 0.0..=self.tmp_data.song_length)
                                .show_value(false),
                        );
                        let current_place_u64 = current_place.round() as u64;
                        if re.changed() && current_place_u64 < conn.status().unwrap().elapsed.unwrap().as_secs() && 
                            conn.rewind(current_place_u64.try_into().unwrap()).is_err() {
                            tracing::error!("Can not rewind to the requested position");
                        } else if re.changed() && current_place_u64 > conn.status().unwrap().elapsed.unwrap().as_secs() &&
                            conn.seek(self.tmp_data.song_pos, current_place_u64.try_into().unwrap()).is_err() 
                        {
                            tracing::error!("Can not seek to the requested position")
                        }         
                    }
                });
            });
        });
        // TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        //     egui::menu::bar(ui, |ui| {

        //         let window_width: f32 = Ui::available_width(ui);

        //         ui.vertical(|ui| {
        //             ui.with_layout(Layout::left_to_right(), |ui| {
        //                 // If there is no song playing do not try and display song title.
        //                 match conn.status().expect("Can not get the mpd state.").state {
        //                     State::Stop => (),
        //                     _ => {
        //                         // Probably a better way to do this but check if the song is the same as last cycle and if not refresh the image
        //                         if self.tmp_data.prev_song.is_empty() {
        //                             self.tmp_data.prev_song.push(conn.currentsong().unwrap().unwrap());
        //                         }
        //                         if self.tmp_data.prev_song[0] != conn.currentsong().unwrap().unwrap() {
        //                             self.song_change(conn);
        //                             self.tmp_data.prev_song[0] = conn.currentsong().unwrap().unwrap();
        //                         }
        //                         // Renders the album art.
        //                         ui.image(
        //                              self.tmp_data.image.texture_id(ctx),
        //                         emath::Vec2 { x: (95. * self.config.scale), y: (95. * self.config.scale) },
        //                         );
        //                         ui.vertical(|ui| {
        //                             ui.label(
        //                                 RichText::new(
        //                                     conn.currentsong().unwrap().unwrap().title.unwrap(),
        //                                 )
        //                                 .color(WHITE)
        //                                 .text_style(body2()),
        //                             );
        //                             ui.label(RichText::new(
        //                                 conn.currentsong().unwrap().unwrap().artist.unwrap(),
        //                             ))
        //                         });
        //                     }
        //                 }

        //                 ui.horizontal(|ui|{
        //                     // Gets the right amount of padding to center the buttons.
        //                     let firstbutton = Ui::available_width(ui) - window_width / 2. - 50. * self.config.scale;
        //                     ui.add_space(firstbutton);
        //                     ui.label(RichText::new("").text_style(heading3()));

        //                     // Button plays the previous song.
        //                     if ui.add(Button::new(RichText::new('玲').text_style(egui::TextStyle::Heading))
        //                         .frame(false)).clicked() {
        //                         conn.prev().unwrap_or(());
        //                         Tangello::song_change(self, conn);
        //                     }

        //                     ui.add_space(1. * self.config.scale);

        //                     // Depending on mpd state render different buttons.
        //                     match conn.status().expect("Can not get the mpd state.").state {
        //                         State::Play => {
        //                             if ui.add(Button::new(RichText::new('').text_style(heading3())).frame(false)).clicked()
        //                             {
        //                                 conn.pause(true)
        //                                     .expect("The pause state could not be toggled.");
        //                             }
        //                         }
        //                         State::Pause => {
        //                             if ui.add(Button::new(RichText::new('').text_style(heading3())).frame(false)).clicked()
        //                             {
        //                                 conn.pause(false)
        //                                     .expect("The pause state could not be toggled.");
        //                             }
        //                         }
        //                         State::Stop => {
        //                             if ui.add(Button::new(RichText::new('').text_style(heading3())).frame(false)).clicked()
        //                             {
        //                                 conn.play().expect("Song could not be played.");
        //                                 Tangello::change_image(self, conn)
        //                             }
        //                         }
        //                     };

        //                     ui.add_space(1. * self.config.scale);

        //                     // Button plays the next song.
        //                     if ui.add(Button::new(RichText::new('怜').text_style(egui::TextStyle::Heading))
        //                         .frame(false)).clicked() {
        //                         conn.next().unwrap_or(());
        //                         Tangello::song_change(self, conn);
        //                 }
        //                 });
        //             });


        //             }
        //         });
        //     })
        // });
    }
}