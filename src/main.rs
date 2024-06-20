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
        let namecheck = RTools::check_file_name_format(&rtools);
        if !namecheck {
            let tag_info = RTools::get_tag_info_mp3(&rtools);
            println!("Tag info: {:?}", tag_info.track);
            if tag_info.track.len() == 1 {
                let new_fn = tag_info.disc
                    + "_0"
                    + &tag_info.track
                    + "_-_"
                    + &tag_info.artist
                    + "_-_"
                    + &tag_info.album
                    + "_-_"
                    + &tag_info.song
                    + &RTools::split_ext(&rtools);
                let newfn = new_fn.replace(" ", "_");
                println!("new_fn: {:?}", newfn);
            } 
            // else {
            //     let new_fn = tag_info.disc
            //         + "_"
            //         + &tag_info.track
            //         + "_-_"
            //         + &tag_info.artist
            //         + "_-_"
            //         + &tag_info.album
            //         + "_-_"
            //         + &tag_info.song
            //         + &RTools::split_ext(&rtools);
            //     let newfn = new_fn.replace(" ", "_");
            //     println!("new_fn: {:?}", newfn);
            // }
        }
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

    pub fn get_tag_info_mp3(&self) -> TagInfoStruct {
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
            }
        };

        let tinfo = TagInfoStruct {
            artist: tag.artist().expect(&self.apath).to_string(),
            album: tag.album().expect(&self.apath).to_string(),
            song: tag.title().expect(&self.apath).to_string(),
            track: tag.track().expect(&self.apath).to_string(),
            disc: tag.disc().unwrap_or(1).to_string(),
            genre: tag.genre().expect(&self.apath).to_string(),
        };

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

        ext
    }

    pub fn split_dir(&self) -> String {
        let path = Path::new(&self.apath);
        let boo_results = path.parent();
        let boo = match boo_results {
            Some(b) => b.to_string_lossy().to_string(),
            None => "split_dir did not work".to_string(),
        };

        boo
    }
}
