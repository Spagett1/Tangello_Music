use eframe::egui::TextStyle;
use eframe::epaint::Color32;
use egui_extras::RetainedImage;
use mpdrs::{Song, Playlist};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use self::fonts::configure_fonts;
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
mod fonts;


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
    filetypes: Vec<String>,
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
            filetypes: vec![".flac".to_string(), ".mp3".to_string(), ".ogg".to_string(), "opus".to_string(), "m4a".to_string(), "aif".to_string(), "aiff".to_string(), "wav".to_string(), "oga".to_string(), "mogg".to_string(), "ogv".to_string(), "ogx".to_string(), "ogm".to_string(), "spx".to_string(), "raw".to_string()],
            scale: 1.,
        }
    }
}
// Tangello contains the other structs in order to have easier access to them.
pub struct Tangello {
    pub config: TangelloConfig,
    pub tmp_data: MyTmpData,
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
