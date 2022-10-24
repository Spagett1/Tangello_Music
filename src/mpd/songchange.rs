use mpdrs::Client;
use notify_rust::{Notification, Timeout};

use crate::graphical::Tangello;

impl Tangello {
    pub fn song_change(&mut self, conn: &mut Client) {
        Tangello::change_image(self, conn);
        self.tmp_data.song_length = conn.currentsong().unwrap().unwrap().duration.unwrap().as_secs_f64();
        self.tmp_data.song_pos = conn.currentsong().unwrap().unwrap().place.unwrap().pos;
        if conn.currentsong().unwrap() == None {
        } else if self.config.notifications {
            let now_playing: String = format!(
                "Now playing: \"{}\"",
                conn.currentsong().unwrap().unwrap().title.as_ref().unwrap()
            );
            if Notification::new()
                .summary("Tangello Music")
                .body(&now_playing[..])
                .timeout(Timeout::Milliseconds(3500))
                .show().is_err() {
                tracing::error!("No notification daemon active, Please disable notifications in the settings");
            };
        }
    }
}