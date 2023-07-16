use chrono::prelude::*;
use glob::{glob_with, MatchOptions};
use lofty::{Probe, TaggedFileExt, ItemKey};
use std::{env, process::exit};

fn print_docs() {
    eprintln!("What To Play Next\n\n");
    eprintln!("Usage: wtpn [flags] [options]\n\n");
    eprintln!("Flags");
    eprintln!("    -h or --help\n");
    eprintln!("Options");
    eprintln!("    -m or --month    <String> ex. \"05\" ");
    eprintln!("    -y or --year     <u8>     ex. 2012");
    eprintln!("    -d or --decade   <u8>     ex. 2010");
    eprintln!("    -o or --original          output original release year");
    eprintln!("    -t or --today             output only records that released today");
    eprintln!("    -g or --genre    <String> ex. \"Rock\"");
}

fn get_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    /*
    *
    *   0 = year
    *   1 = decade
    *   2 = month
    *   3 = original
    *   4 = today
    *   5 = genre
    *
    */
    let mut args_key: Vec<String> = vec!("".to_string() ;6);

    // Print documentation
    if args.iter().any(|e| e == "-h") || args.iter().any(|e| e == "--help") {
        print_docs();
        exit(1);
    }

    for (key, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "-y" | "--year" => args_key[0] = get_arg_value(&args, key),
            "-d" | "--decade" => args_key[1] = get_arg_value(&args, key),
            "-m" | "--month" => args_key[2] = get_arg_value(&args, key),
            "-g" | "--genre" => args_key[5] = get_arg_value(&args, key),
            "-o" | "--original" => args_key[3] = String::from("1"),
            "-t" | "--today" => args_key[4] = String::from("1"),
            _ => continue,
        }
    }

    args_key
}

fn get_arg_value(args: &Vec<String>, key: usize) -> String {
    // In case if the flag is at the end of the Vec
    if key == (args.len() - 1) {
        println!("{} is missing value", args[key]);
        exit(1);
    }

    // if I want to convert values into different types
    let value: String = args[key+1].to_string();

    // If the next key value is anything including a flag or empty then stop the program
    if value.is_empty() || value.contains('-') || value.contains("--") {
        println!("{} is missing value", args[key]);
        exit(1);
    }

    value
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home = env::var_os("HOME").unwrap().into_string().unwrap();
    let mut list_of_songs: Vec<String> = vec![];
    let args = get_args();

    let globs = glob_with(&(home + "/music/*/*/01-*"), MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    }).unwrap();

    // TODO: add a way of getting the current month and day
    let dt: DateTime<Local> = chrono::offset::Local::now();

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
        let artist = tag.get_string(&ItemKey::TrackArtist).unwrap();
        let album = tag.get_string(&ItemKey::AlbumTitle).unwrap();
        let genres = tag.get_string(&ItemKey::Genre).unwrap();

        let mut vec_date: Vec<&str> = date.split('-').collect();

        if !args[0].is_empty() && args[0] != vec_date[0] {
            continue;
        }

        if !args[1].is_empty() &&  args[1] != vec_date[0] {
            continue;
        }

        if !args[2].is_empty() && args[2] != vec_date[1] {
            continue;
        }

        if !args[5].is_empty() && !genres.contains(&args[5]) {
            continue;
        }

        let month = &dt.format("%m").to_string();
        let day = &dt.format("%d").to_string();
        let today = &dt.format("%Y-%m-%d").to_string();

        if args[3].is_empty() {
            if vec_date[1] >= month {
                if vec_date[1] == month && vec_date[2] < day {
                    vec_date[0] = "2024";
                } else {
                    vec_date[0] = "2023";
                }
            } else {
                vec_date[0] = "2024";
            }
        }

        date = format!("{}-{}-{}", vec_date[0], vec_date[1], vec_date[2]);

        if args[4] == "1" && today != &date {
            continue;
        }

        list_of_songs.push(format!("{} {} - {}", date, artist, album));
    }

    list_of_songs.sort();

    for song in list_of_songs {
        println!("{}", song);
    }

    Ok(())
}
