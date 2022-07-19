use eframe::egui::{
    self, FontData, FontDefinitions, TextStyle};
use eframe::epaint::{Color32, FontId};
use egui::FontFamily;
use egui_extras::RetainedImage;
use mpdrs::{Song, Playlist};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
const PADDING: f32 = 5.0;
const BLUE: Color32 = Color32::from_rgb(20, 177, 255);
const WHITE: Color32 = Color32::from_rgb(190, 190, 190);
mod top_panel;
mod settings;
mod sidebar;
mod data;
mod confirm;
mod queue;
mod library;
mod playlist;
mod footer;
mod playlistadd;

#[derive(Serialize, Deserialize)]
// This struct contains elements that will persist in the settings configuration file.
pub struct TangelloConfig {
    pub dark_mode: bool,
    pub notifications: bool,
    pub mpd_address: String,
    // Music path is needed because mpd-rs seems to have insufficient perms to do it
    // This is a workaround and i would like to clean it up if possible.
    pub music_path: PathBuf,
    tmp_music_path: String,
    tmp_address: String,
    scale: f32,
}

// This struct contains values that will reset each time the program is closed (used to see what states windows are in, etc.)
pub enum View {
    Queue,
    Library,
    Playlist,
    AddToPlaylist,
}
pub struct MyTmpData {
    pub settings_open: bool,
    pub sidebar_open: bool,
    panel_size: f32,
    pub first_run: bool,
    pub image: RetainedImage,
    songlist_vec: Vec<Song>,
    songs: Vec<Song>,
    prev_song: Vec<Song>,
    pub view: View,
    search: String,
    search_bar: bool,
    search_bar_want_focus: bool,
    selected_playlist: Vec<Playlist>,
    confirm_open: bool,
    new_playlist_name: String,
    new_playlist_open: bool,
    add_playlist_songs: Vec<Song>
}
// Defines the default values for the temporary data
impl Default for MyTmpData {
    fn default() -> Self {
        Self {
            settings_open: false,
            sidebar_open: false,
            panel_size: 0.,
            first_run: true,
            image: RetainedImage::from_image_bytes("Cover_Art", include_bytes!("../../assets/empty.png"))
                .unwrap(),
            songlist_vec: vec![],
            songs: vec![],
            prev_song: vec![],
            view: View::Queue,
            search: "".to_string(),
            search_bar: false,
            search_bar_want_focus: true,
            selected_playlist: vec![],
            confirm_open: false,
            new_playlist_name: "".to_string(),
            new_playlist_open: false,
            add_playlist_songs: vec![],
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
            scale: 1.,
        }
    }
}
// Tangello contains the other structs in order to have easier access to them.
pub struct Tangello {
    pub config: TangelloConfig,
    pub tmp_data: MyTmpData,
}

fn configure_fonts(config: &TangelloConfig,ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    let mut style = (*ctx.style()).clone();
    // Imports the MesloLGS font from its ttf file in order to enable support for other characters
    fonts.font_data.insert(
        "MesloLGS".to_owned(),
        FontData::from_static(include_bytes!("../../assets/MesloLGS_NF_Regular.ttf")),
    );
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .push("MesloLGS".to_owned());

    // Sets font sizes for the different Text Styles.
    style.text_styles = [
        (TextStyle::Heading, FontId::new(35.0 * config.scale, FontFamily::Proportional)),
        (TextStyle::Body, FontId::new(20.0 * config.scale, FontFamily::Proportional)),
        (body2(), FontId::new(25.0 * config.scale, FontFamily::Proportional)),
        (heading2(), FontId::new(27.0 * config.scale, FontFamily::Proportional)),
        (heading3(), FontId::new(50.0 * config.scale, FontFamily::Proportional)),
        (TextStyle::Monospace,FontId::new(14.0 * config.scale, FontFamily::Proportional)),
        (TextStyle::Button,FontId::new(30.0 * config.scale, FontFamily::Proportional)),
        (TextStyle::Small,FontId::new(10.0 * config.scale, FontFamily::Proportional)),
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

        configure_fonts(&config, &cc.egui_ctx);

        Tangello { config, tmp_data }
    }
}
