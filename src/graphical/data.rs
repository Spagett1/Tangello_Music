use mpdrs::Client;

use super::Tangello;

impl Tangello {
    // Searches recursively through the directories for songs.
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
                // Make sure files actually have extensions.
                i.1.contains(".") && 
                // Ignore cover.jpg type files
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
    pub fn grab_lib_data(&mut self, conn: &mut Client) {
        self.tmp_data.songlist_vec.clear();
        match conn.update() {_ => ()}
        let mut path = "".to_string();
        self.tmp_data.songlist_vec.clear(); 
        Tangello::search_dirs(self, conn, &mut path);
    }
}