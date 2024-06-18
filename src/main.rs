use id3::{Tag, TagLike};
use std::env;
// use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a directory path as an argument.");
        return;
    }

    let dir_path = &args[1];
    if !Path::new(dir_path).is_dir() {
        println!("The provided path is not a directory.");
        return;
    }

    let rtools = RTools {
        apath: dir_path.to_string(),
    };

    let mp3_list = RTools::find_media(&rtools);

    for mp3 in mp3_list {
        let rtools = RTools { apath: mp3 };
        let tag_info = RTools::get_tag_info_mp3(&rtools);
        println!("artist: {:?}", tag_info.artist);
        // match tag_info {
        //     Ok((artist, album, song, track, disc, genre)) => {
        //         println!(
        //             "Artist: {}, Album: {}, Song: {}, Track: {}, Disc: {}, Genre: {}",
        //             artist, album, song, track, disc, genre
        //         );
        //     }
        //     Err(e) => {
        //         println!("Error: {}", e);
        //     }
        // }
    }
}

#[derive(Debug)]
pub struct RTools {
    pub apath: String,
}

#[derive(Debug)]
pub struct TagInfoStruct {
    pub artist: String,
    pub album: String,
    pub song: String,
    pub track: String,
    pub disc: String,
    pub genre: String,
}

impl RTools {
    // pub fn split_home_dir(&self) -> String {
    //     let path = Path::new(&self.apath);
    //     let home_dir = Path::new("/home/charliepi/Music");

    //     let relative_path = path.strip_prefix(home_dir).unwrap_or(path);
    //     let file_name = relative_path
    //         .file_name()
    //         .unwrap_or_else(|| &std::ffi::OsStr::new(""));

    //     file_name.to_string_lossy().into_owned()
    // }
    pub fn find_media(&self) -> Vec<String> {
        println!("Dir path: {:?}", &self.apath);
        let mut media_files = Vec::new();
        for entry in WalkDir::new(&self.apath) {
            let entry = entry.unwrap();
            if entry
                .path()
                .extension()
                .map_or(false, |ext| ext == "mp3" || ext == "MP3")
            {
                media_files.push(entry.path().to_string_lossy().into_owned());
            }
        }

        media_files
    }

    pub fn check_file_name_format(&self) -> bool {
        let re = regex::Regex::new(r"\d+_\d\d+_-_.+_-_.+_-_.+\.mp3").unwrap();
        let re2 = regex::Regex::new(r"\d+_\d\d+_-_.+_-_.+_-_.+\.flac").unwrap();
        let re3 = regex::Regex::new(r"\d+_\d\d+_-_.+_-_.+_-_.+\.ogg").unwrap();
        let re4 = regex::Regex::new(r"\d+_\d\d+_-_.+_-_.+_-_.+\.wav").unwrap();
        let file_name = Path::new(&self.apath)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        if re.is_match(file_name) {
            return true;
        } else if re2.is_match(file_name) {
            return true;
        } else if re3.is_match(file_name) {
            return true;
        } else if re4.is_match(file_name) {
            return true;
        } else {
            return false;
        }
    }

    // pub fn split_base_dir_filename(&self) -> (String, String) {
    //     let path = Path::new(&self.apath);
    //     let dir_path = path.parent().unwrap();
    //     let filename = path.file_name().unwrap();

    //     (
    //         dir_path.to_string_lossy().to_string(),
    //         filename.to_string_lossy().to_string(),
    //     )
    // }

    // pub fn split_artist_album(&self) -> (String, String) {
    //     let path = Path::new(&self.apath);
    //     let basedir = path.parent().unwrap();
    //     let basedirpath = Path::new(&basedir);
    //     let album = basedirpath.file_name().unwrap();
    //     let basedirpath2 = basedirpath.parent().unwrap();
    //     let bdp3 = Path::new(&basedirpath2);
    //     let artist = bdp3.file_name().unwrap();
    //     let album_string = album.to_string_lossy().to_string();
    //     let artist_string = artist.to_string_lossy().to_string();

    //     let album_final = album_string.replace("_", " ");
    //     let artist_final = artist_string.replace("_", " ");

    //     (artist_final, album_final)
    // }

    pub fn get_tag_info_mp3(
        &self,
    ) -> TagInfoStruct {
        let tag = match Tag::read_from_path(&self.apath) {
            Ok(tag) => tag,
            Err(_) => {
                println!("No ID3 tag found for: {:?}", &self.apath);
                return TagInfoStruct {
                    artist: "No ID3 tag found".to_string(),
                    album: "No ID3 tag found".to_string(),
                    song: "No ID3 tag found".to_string(),
                    track: "No ID3 tag found".to_string(),
                    disc: "No ID3 tag found".to_string(),
                    genre: "No ID3 tag found".to_string(),
                };
                // let target_dir = Path::new("/home/charliepi/needs_work");
                // if !target_dir.exists() {
                //     fs::create_dir_all(target_dir)?;
                // }
                // fs::rename(
                //     &self.apath,
                //     target_dir.join(Path::new(&self.apath).file_name().unwrap()),
                // )?;
                // return Err(std::io::Error::new(
                //     std::io::ErrorKind::Other,
                //     "No ID3 tag found",
                // ));
            }
        };

        let tinfo = TagInfoStruct {
            artist: tag.artist().expect(&self.apath).to_string(),
            album: tag.album().expect(&self.apath).to_string(),
            song: tag.title().expect(&self.apath).to_string(),
            track: tag.track().expect(&self.apath).to_string(),
            disc: tag.disc().expect("01").to_string(),
            genre: tag.genre().expect(&self.apath).to_string(),
        };
        // let artist = tag.artist().expect(&self.apath);
        // let album = tag.album().expect(&self.apath);
        // let song = tag.title().expect(&self.apath);
        // let track = tag.track().expect(&self.apath);
        // let disc = tag.disc().expect(&self.apath);
        // let genre = tag.genre().expect(&self.apath);

        // Ok((
        //     artist.to_string(),
        //     album.to_string(),
        //     song.to_string(),
        //     track.to_string(),
        //     disc.to_string(),
        //     genre.to_string(),
        // ))

        tinfo
    }

    pub fn split_ext(&self) -> String {
        let path = Path::new(&self.apath);
        let boo_results = path.extension();
        let boo = match boo_results {
            Some(b) => b.to_string_lossy().to_string(),
            None => "split_ext did not work".to_string(),
        };
        let ext = ".".to_string() + boo.as_str();
        println!("Ext: {:?}", ext);

        ext
    }

    // pub fn get_dims(&self) -> (u32, u32) {
    //     let dims = get_image_dims(&self.apath);

    //     dims
    // }
    // pub fn artist_starts_with(&self) -> String {
    //     let tag = Tag::read_from_path(&self.apath).expect(&self.apath);
    //     let artist = tag.artist().expect(&self.apath);
    //     let first_letter = artist.chars().next().unwrap();

    //     first_letter.to_string()
    // }

    // pub fn album_starts_with(&self) -> String {
    //     let tag = Tag::read_from_path(&self.apath).expect(&self.apath);
    //     let album = tag.album().expect(&self.apath);
    //     let first_letter = album.chars().next().unwrap();

    //     first_letter.to_string()
    // }

    // pub fn song_starts_with(&self) -> String {
    //     let tag = Tag::read_from_path(&self.apath).expect(&self.apath);
    //     let song = tag.title().expect(&self.apath);
    //     let first_letter = song.chars().next().unwrap();

    //     first_letter.to_string()
    // }

    // pub fn create_mp3_play_path(&self) -> String {
    //     let psplit = self.apath.split("/").skip(3).collect::<Vec<&str>>();
    //     let assend = psplit.join("/");

    //     let myhttpd = env::var("RUSIC_HTTP_ADDR").unwrap();
    //     let myport = env::var("RUSIC_PORT").unwrap();

    //     // let playpath = myhttpd + &myport + "/Music/" + assend.as_str();
    //     let playpath = myhttpd + &myport + "/" + assend.as_str();

    //     playpath
    // }
}
