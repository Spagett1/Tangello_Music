use crate::graphical::Tangello;

use mpdrs::Client;

use lofty::Probe;
use egui_extras::RetainedImage;
impl Tangello {
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
                    RetainedImage::from_image_bytes("Cover_Art", include_bytes!("../../assets/empty.png"))
                        .unwrap()
            }
        };
    }

}