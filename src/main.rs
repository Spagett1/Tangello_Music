mod graphical;
mod mpd;
use eframe::egui::{self, Visuals};
use eframe::NativeOptions;
use graphical::Tangello;
use mpdrs::Client;

impl eframe::App for Tangello {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        // Define the mpd address and variable which allows us to interact with mpd.
        let address = self.config.mpd_address.clone();
        let mut conn = Client::connect(address).expect(
            "No mpd server found at this address, remember the default is generally 127.0.0.1:6600",
        );
        // match Client::connect(address) {
        //     Ok(_) => let mut conn = Client::connect(address).unwrap(),
        //     Err(_) => panic!(),
        // };
        // Read the settings for whether the user has set light more or dark mode.
        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        // Render the elements of the program.

        // self.render_top_panel(ctx, frame);
        self.render_top_panel(ctx, frame);

        // Read what the state of the sidebar is supposed to be and render that depending on the state.
        if self.tmp_data.sidebar_open {
            // self.render_sidebar(&mut conn, ctx);
            Tangello::render_sidebar(self, &mut conn, ctx);
        }
        // Render the main window
        match self.tmp_data.view {
            graphical::View::Queue => self.render_queue(&mut conn, ctx),
            graphical::View::Library => self.render_library(&mut conn, ctx),
            graphical::View::Playlist => self.render_playlist(&mut conn, ctx),
            graphical::View::AddToPlaylist => self.playlist_add(&mut conn, ctx)
        }

        // Check whether to render settings
        if self.tmp_data.settings_open {
            self.render_settings(ctx);
        }

        self.render_footer(ctx, &mut conn);

        // If this is the first pass of the update function (when the program is opened) update the albumart.
        if self.tmp_data.first_run {
            Tangello::song_change(self, &mut conn);
            self.tmp_data.first_run = false;
        }
    }
}

fn main() {
    let options = NativeOptions::default();
    eframe::run_native(
        "Tangello Music",
        options,
        Box::new(|cc| Box::new(Tangello::new(cc))),
    );
}
