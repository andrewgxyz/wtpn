use glob::{glob_with, MatchOptions};
use lofty::{Probe, TaggedFileExt, ItemKey};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home = env::var_os("HOME").unwrap().into_string().unwrap();

    let globs = glob_with(&(home + "/music/*/*/01-*"), MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    }).unwrap();

    for entry in globs.flatten() {
        let filename = entry.display().to_string();

        let tagged_file = Probe::open(filename)
            .expect("ERROR: Bad path provided!")
            .read()
            .expect("ERROR: Failed to read file!");

        let tag = match tagged_file.primary_tag() {
            Some(primary) => primary,
            None => tagged_file.first_tag().expect("ERROR: No tags found!")
        };

        let mut date = tag.get_string(&ItemKey::RecordingDate).map(|s| s.to_string()).unwrap();

        let mut vec_date: Vec<&str> = date.split("-").collect();

        if vec_date[1] >= "05" {
            vec_date[0] = "2023";
        } else {
            vec_date[0] = "2024";
        }

        date = format!("{}-{}-{}", vec_date[0], vec_date[1], vec_date[2]);
        let artist = tag.get_string(&ItemKey::TrackArtist).unwrap();
        let album = tag.get_string(&ItemKey::AlbumTitle).unwrap();

        println!("{} {} - {}", date, artist, album);
    }


    Ok(())
}
