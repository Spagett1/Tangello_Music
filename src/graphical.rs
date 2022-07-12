use eframe::egui::{
    self, Button, CentralPanel, FontData, FontDefinitions, Layout, RichText, ScrollArea, Separator,
    SidePanel, TextEdit, TextStyle, TopBottomPanel, Ui, Window,
};
use eframe::emath;
use eframe::epaint::{Color32, FontId};
use egui::FontFamily;
use egui_extras::RetainedImage;
use mpdrs::{Client, Song, State, Playlist};
use notify_rust::{Notification, Timeout};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap};
use std::path::PathBuf;
const PADDING: f32 = 5.0;
const BLUE: Color32 = Color32::from_rgb(20, 177, 255);
const SLIDER_COLOUR: Color32 = Color32::from_rgb(70, 70, 70);
const WHITE: Color32 = Color32::from_rgb(190, 190, 190);
use lofty::Probe;

#[derive(Serialize, Deserialize)]
// This struct contains elements that will persist in the settings configuration file.
pub struct TangelloConfig {
    pub dark_mode: bool,
    notifications: bool,
    pub mpd_address: String,
    // Music path is needed because mpd-rs seems to have insufficient perms to do it
    // This is a workaround and i would like to clean it up if possible.
    music_path: PathBuf,
    tmp_music_path: String,
    tmp_address: String,
}

// This struct contains values that will reset each time the program is closed (used to see what states windows are in, etc.)
pub enum View {
    Queue,
    Library,
    Playlist,
}
pub struct MyTmpData {
    settings_open: bool,
    pub sidebar_open: bool,
    panel_size: f32,
    pub first_run: bool,
    image: RetainedImage,
    songlist_vec: Vec<Song>,
    songs: Vec<Song>,
    prev_song: Vec<Song>,
    pub view: View,
    search: String,
    search_bar: bool,
    search_bar_want_focus: bool,
    selected_playlist: Vec<Playlist>
}
// Defines the default values for the temporary data
impl Default for MyTmpData {
    fn default() -> Self {
        Self {
            settings_open: false,
            sidebar_open: false,
            panel_size: 0.,
            first_run: true,
            image: RetainedImage::from_image_bytes("Cover_Art", include_bytes!("../assets/empty.png"))
                .unwrap(),
            songlist_vec: vec![],
            songs: vec![],
            prev_song: vec![],
            view: View::Queue,
            search: "".to_string(),
            search_bar: false,
            search_bar_want_focus: true,
            selected_playlist: vec![]
        }
    }
}

// Defines the defaults for the persistant data, this will be overwritten by the config file.
impl Default for TangelloConfig {
    fn default() -> Self {
        Self {
            dark_mode: true,
            notifications: true,
            music_path: dirs::audio_dir().unwrap(),
            mpd_address: "127.0.0.1:6600".to_string(),
            tmp_music_path: dirs::audio_dir()
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap()
                .to_string(),
            tmp_address: "127.0.0.1:6600".to_string(),
        }
    }
}
// Tangello contains the other structs in order to have easier access to them.
pub struct Tangello {
    pub config: TangelloConfig,
    pub tmp_data: MyTmpData,
}

fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    let mut style = (*ctx.style()).clone();
    // Imports the MesloLGS font from its ttf file in order to enable support for other characters
    fonts.font_data.insert(
        "MesloLGS".to_owned(),
        FontData::from_static(include_bytes!("../assets/MesloLGS_NF_Regular.ttf")),
    );
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .push("MesloLGS".to_owned());

    // Sets font sizes for the different Text Styles.
    style.text_styles = [
        (
            TextStyle::Heading,
            FontId::new(35.0, FontFamily::Proportional),
        ),
        (TextStyle::Body, FontId::new(20.0, FontFamily::Proportional)),
        (body2(), FontId::new(25.0, FontFamily::Proportional)),
        (heading2(), FontId::new(27.0, FontFamily::Proportional)),
        (heading3(), FontId::new(50.0, FontFamily::Proportional)),
        (TextStyle::Monospace,FontId::new(14.0, FontFamily::Proportional)),
        (TextStyle::Button,FontId::new(30.0, FontFamily::Proportional)),
        (TextStyle::Small,FontId::new(10.0, FontFamily::Proportional)),
    ].into();
    ctx.set_style(style);
    ctx.set_fonts(fonts);
}
// Creates some new Text Styles so i can have more font size variation.
fn body2() -> TextStyle {
    TextStyle::Name("SettingsBody".into())
}

fn heading2() -> TextStyle {
    TextStyle::Name("SettingsHeading".into())
}

fn heading3() -> TextStyle {
    TextStyle::Name("PlayButton".into())
}

impl Tangello {
    // This is run once at the beggining of the program
    pub fn new(cc: &eframe::CreationContext<'_>) -> Tangello {
        // Shortens the config and tmp_data so we can write it easier
        let config: TangelloConfig = confy::load("tangello").unwrap_or_default();
        let tmp_data: MyTmpData = MyTmpData::default();

        configure_fonts(&cc.egui_ctx);

        Tangello { config, tmp_data }
    }

    // Contains the egui data for the top panel.
    pub fn render_top_panel(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(7.);
            egui::menu::bar(ui, |ui| {
                let sidebar_btn =
                    ui.button(RichText::new("  ").text_style(egui::TextStyle::Heading));
                // Sets the sidebar_open value, this decides whether to open the sidebar or close it.
                if sidebar_btn.clicked() && !self.tmp_data.sidebar_open {
                    self.tmp_data.sidebar_open = true;
                } else if sidebar_btn.clicked() && self.tmp_data.sidebar_open {
                    self.tmp_data.sidebar_open = false;
                }
                ui.add_space(Ui::available_width(ui) / 2. - 120.);
                ui.heading("Tangello Music");

                ui.with_layout(Layout::right_to_left(), |ui| {
                    // Adds a close button and quits the program when pressed
                    if ui.add(Button::new("❌")).clicked() {
                        frame.quit();
                    }

                    // Sets the settings open value to true
                    if ui.add(Button::new("漣")).clicked() {
                        self.tmp_data.settings_open = self.render_settings(ctx);
                    } else if self.tmp_data.settings_open {
                        self.render_settings(ctx);
                    }
                })
            });
            ui.add_space(10.);
        });
    }

    // Searches recursively throught the directories for songs.
    fn search_dirs(&mut self, conn: &mut Client, path: &mut String) {
        for i in conn.listfiles(path.clone().as_str()).unwrap().iter() {
            if i.0 == "directory" {
                let og_path = path.clone();
                if path == "" {
                    path.push_str(i.1.as_str());
                } else {
                    let tmp = format!("/{}", i.1);
                    path.push_str(tmp.as_str());
                }
                Tangello::search_dirs(self, conn, path);
                path.clear();
                *path = og_path;
            } 
            else if i.0 == "file" && 
                i.1.contains(".") && 
                !i.1.contains(".jpg") && 
                !i.1.contains(".png") {

                let song_path: String = if path == "" {
                    i.1.clone()
                } else {
                    format!("{}/{}", path, i.1)
                };

                for i in conn.lsinfo(song_path.as_str()).unwrap() {
                    match i {
                        mpdrs::lsinfo::LsInfoResponse::Song(song) => {
                            self.tmp_data.songlist_vec.push(song)
                        },
                        _ => ()
                    }
                }
            }
        }
    }

    // Grabs a vector of every song in the users music library
    fn grab_lib_data(&mut self, conn: &mut Client) {
        self.tmp_data.songlist_vec.clear();
        match conn.update() {_ => ()}
        let mut path = "".to_string();
        self.tmp_data.songlist_vec.clear(); 
        Tangello::search_dirs(self, conn, &mut path);
    }

    pub fn render_playlist(&mut self, conn: &mut Client, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            let current_playlist= self.tmp_data.selected_playlist[0].clone();
            egui::menu::bar(ui, |ui|{
                ui.vertical_centered(|ui|{
                    ui.label(RichText::new(&current_playlist.name).text_style(heading2()));
                });

            });
            ui.add_space(3.5);
            ui.separator();
            ScrollArea::vertical()
                .max_height(ui.available_height() - 63.)
                .show(ui, |ui| {

                    let mut pos = 0;
                    for song in conn.playlist(current_playlist.name.as_str()).unwrap() {
                        pos += 1;
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
                                match conn.pl_delete(&current_playlist.name, pos - 1) {
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
                            [70., 10.],
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
    // This renders the queue, this will be replaced with other views like playlist view later
    pub fn render_queue(&mut self, conn: &mut Client, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new("Queue").text_style(heading2()));
                });

                ui.with_layout(Layout::right_to_left(), |ui| {
                    ui.add_space(20.);
                    if ui.add(Button::new("")).clicked() {
                        match conn.clear() {
                            Err(_) => tracing::error!("Cannot clear the queue."),
                            Ok(_) => (),
                        }
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
                                ui.add(
                                    Button::new(RichText::new(album).color(BLUE))
                                        .frame(false)
                                        .small(),
                                );
                            });
                        });
                        ui.add_space(PADDING);
                        ui.add(Separator::default());
                    }
                    ui.add_space(40.);
                });
        });
    }

    // This function updates the image.
    pub fn change_image(&mut self, conn: &mut Client) {
        // Checks if there is a song playing.
        if conn.currentsong().unwrap() == None {
        } else {
            // Grabs the overall music path which is set by the user.
            let mut path = self.config.music_path.clone();
            // Adds on the path of the song itself.
            path.push(conn.currentsong().unwrap().unwrap().file);

            // Opens up the file for use with lofty to grab the tags.
            let tagged_file = Probe::open(path)
                .expect("bad path")
                .read(true)
                .expect("Failed to read file");

            // If no tags are found return None.
            let tag = match tagged_file.primary_tag() {
                Some(primary_tag) => primary_tag,
                None => tagged_file.first_tag().expect("ERROR: No tags found!"),
            };

            // Check if the song has a number of pictures.
            if tag.picture_count() >= 1 {
                // Set the album art image
                self.tmp_data.image =
                    RetainedImage::from_image_bytes("debug_name", tag.pictures()[0].data())
                        .expect("No image");
            } else {
                // If the song has no album art then set the placeholder image.
                self.tmp_data.image =
                    RetainedImage::from_image_bytes("Cover_Art", include_bytes!("../assets/empty.png"))
                        .unwrap()
            }
        };
    }

    // This is run every time a song changes, stuff like sending a notification and changing the image.
    pub fn song_change(&mut self, conn: &mut Client) {
        Tangello::change_image(self, conn);
        if conn.currentsong().unwrap() == None {
        } else if self.config.notifications {
            let now_playing: String = format!(
                "Now playing: \"{}\"",
                conn.currentsong().unwrap().unwrap().title.as_ref().unwrap()
            );
            match Notification::new()
                .summary("Tangello Music")
                .body(&now_playing[..])
                .timeout(Timeout::Milliseconds(3500))
                .show()
            {
                Err(_) => tracing::error!("No notification daemon active, Please disable notifications in the settings"),
                Ok(_) => (),
            };
        }
    }

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
                    for i in conn.playlists().unwrap() {
                        ui.horizontal(|ui|{
                            if ui.add(Button::new(&i.name).small().frame(false)).clicked() {
                                self.tmp_data.selected_playlist.clear();
                                self.tmp_data.selected_playlist.push(i);
                                self.tmp_data.view = View::Playlist;
                            };
                        });
                    }
                });
                panel_width
            })
            .inner;
        true
    }

    // Renders the footer with the sliders, info on songs playing, etc.
    pub fn render_footer(&mut self, ctx: &egui::Context, conn: &mut Client) {
        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // Renders the album art.

                let window_width: f32 = Ui::available_width(ui);

                ui.vertical(|ui| {
                    ui.with_layout(Layout::left_to_right(), |ui| {
                        // If there is no song playing do not try and display song title.

                        match conn.status().expect("Can not get the mpd state.").state {
                            State::Stop => (),
                            _ => {
                                // Probably a better way to do this but check if the song is the same as last cycle and if not refresh the image
                                if self.tmp_data.prev_song.len() == 0 {
                                    self.tmp_data.prev_song.push(conn.currentsong().unwrap().unwrap());
                                }
                                if self.tmp_data.prev_song[0] != conn.currentsong().unwrap().unwrap() {
                                    self.song_change(conn);
                                    self.tmp_data.prev_song[0] = conn.currentsong().unwrap().unwrap();
                                }
                                ui.image(
                                     self.tmp_data.image.texture_id(ctx),
                                emath::Vec2 { x: (90.), y: (90.) },
                                );
                                ui.vertical(|ui| {
                                    ui.label(
                                        RichText::new(
                                            conn.currentsong().unwrap().unwrap().title.unwrap(),
                                        )
                                        .color(WHITE)
                                        .text_style(body2()),
                                    );
                                    ui.label(RichText::new(
                                        conn.currentsong().unwrap().unwrap().artist.unwrap(),
                                    ))
                                });
                            }
                        }

                        // Gets the right amount of padding to centre the buttons.
                        let firstbutton = Ui::available_width(ui) - window_width / 2. - 50.;
                        ui.add_space(firstbutton);
                        ui.label(RichText::new("").text_style(heading3()));

                        // Button plays the previous song.
                        if ui
                            .add(
                                Button::new(
                                    RichText::new('玲').text_style(egui::TextStyle::Heading),
                                )
                                .frame(false),
                            )
                            .clicked()
                        {
                            conn.prev().unwrap_or(());
                            Tangello::song_change(self, conn);
                        }

                        // Depending on mpd state render different buttons.
                        match conn.status().expect("Can not get the mpd state.").state {
                            State::Play => {
                                if ui.add(Button::new(RichText::new('').text_style(heading3())).frame(false)).clicked()
                                {
                                    conn.pause(true)
                                        .expect("The pause state could not be toggled.");
                                }
                            }
                            State::Pause => {
                                if ui.add(Button::new(RichText::new('').text_style(heading3())).frame(false)).clicked()
                                {
                                    conn.pause(false)
                                        .expect("The pause state could not be toggled.");
                                }
                            }
                            State::Stop => {
                                if ui.add(Button::new(RichText::new('').text_style(heading3())).frame(false)).clicked()
                                {
                                    conn.play().expect("Song could not be played.");
                                    Tangello::change_image(self, conn)
                                }
                            }
                        };

                        // Button plays the next song.
                        if ui
                            .add(
                                Button::new(
                                    RichText::new('怜').text_style(egui::TextStyle::Heading),
                                )
                                .frame(false),
                            )
                            .clicked()
                        {
                            conn.next().unwrap_or(());
                            Tangello::song_change(self, conn);
                        }
                    });

                    // Sets the slider colours and spacing
                    Ui::style_mut(ui).visuals.widgets.inactive.bg_fill = SLIDER_COLOUR;
                    Ui::style_mut(ui).spacing.slider_width = window_width;

                    if conn.currentsong().unwrap() == None {
                        // If there is no song playing just show a dummy slider.
                        let mut dummy_size: f32 = 0.0;
                        ui.add(
                            egui::widgets::Slider::new(&mut dummy_size, 0.0..=100.0)
                                .show_value(false),
                        );
                    } else {
                        // Repaints the slider, this is needed otherwise the slider only updates when the mouse is on it.
                        if conn.status().expect("can not get the mpd state.").state == State::Play {
                            ctx.request_repaint();
                        }
                        // Grabs the song length and the current position of the song.
                        let song_length = conn.status().unwrap().duration.unwrap().as_secs();
                        let song_pos = conn.currentsong().unwrap().unwrap().place.unwrap().pos;
                        let mut current_place = conn.status().unwrap().elapsed.unwrap().as_secs();
                        let re = ui.add(
                            egui::widgets::Slider::new(&mut current_place, 0..=song_length)
                                .show_value(false),
                        );
                        if re.changed() && current_place < conn.status().unwrap().elapsed.unwrap().as_secs() {
                            match conn.rewind(current_place.try_into().unwrap()) {
                                Err(_) => {
                                    tracing::error!("Can not rewind to the requested position")
                                }
                                Ok(_) => (),
                            }
                        } else if re.changed() && current_place > conn.status().unwrap().elapsed.unwrap().as_secs()
                        {
                            match conn.seek(song_pos, current_place.try_into().unwrap()) {
                                Err(_) => tracing::error!("Can not seek to the requested position"),
                                Ok(_) => (),
                            }
                        }
                    }
                });
            })
        });
    }

    fn render_settings(&mut self, ctx: &egui::Context) -> bool {
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
            });
        // Return that the settings are open
        true
    }
}
