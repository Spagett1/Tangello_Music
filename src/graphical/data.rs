use mpdrs::Client;

use super::Tangello;

impl Tangello {
    // Searches recursively through the directories for songs.
    fn search_dirs(&mut self, conn: &mut Client, path: &mut String) {
        for i in conn.listfiles(path.clone().as_str()).unwrap().iter() {
            if i.0 == "directory" {
                let og_path = path.clone();
                if path.is_empty() {
                    path.push_str(i.1.as_str());
                } else {
                    let tmp = format!("/{}", i.1);
                    path.push_str(tmp.as_str());
                }
                Tangello::search_dirs(self, conn, path);
                path.clear();
                *path = og_path;
            } 
            else if i.0 == "file" && self.config.filetypes.iter().any(|end| i.1.contains(end)) {
                let song_path: String = if path.is_empty() {
                    i.1.clone()
                } else {
                    format!("{}/{}", path, i.1)
                };

                for i in conn.lsinfo(song_path.as_str()).unwrap() {
                    if let mpdrs::lsinfo::LsInfoResponse::Song(song) = i {
                        self.tmp_data.songlist_vec.push(song)
                    }
                }
            }
        }
    }

    // Grabs a vector of every song in the users music library
    pub fn grab_lib_data(&mut self, conn: &mut Client) {
        self.tmp_data.songlist_vec.clear();
        if conn.update().is_err() {
            tracing::error!("Could not update the mpd database.")
        };
        let mut path = "".to_string();
        self.tmp_data.songlist_vec.clear(); 
        Tangello::search_dirs(self, conn, &mut path);
    }
}