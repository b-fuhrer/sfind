mod config;

use clap::{Arg, ArgMatches, command};
use config::{Config, ConfigType, ContentInfo, FileInfo};
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::{fs, io, process};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct File {
    pub path: PathBuf,
}

fn parse_input() -> ArgMatches {
    command!()
        .arg(
            Arg::new("start_dir")
                .short('d')
                .long("dir")
                .aliases([
                    "directory",
                    "start",
                    "startdir",
                    "startdirectory",
                    "start-dir",
                    "start-directory",
                    "sd",
                    "sdir",
                ])
                .required(false)
                .help("Start directory where the find should start its recursive descent.")
                .default_value("."),
        )
        .arg(
            Arg::new("file_name")
                .short('f')
                .long("file")
                .aliases(["filename", "fn"])
                .required(false)
                .help("Substring that must be contained in the file name searched for."),
        )
        .arg(
            Arg::new("content")
                .short('c')
                .long("content")
                .required(false)
                .help("Substring that must be contained in the contents of the file."),
        )
        .get_matches()
}

fn retrieve_config_matches(config: &Config) -> Vec<File> {
    let dir_entries = WalkDir::new(&config.starting_directory);

    // TODO: implement optional visibility of io::Error instead of always discarding in .filter_map()
    match &config.config_type {
        ConfigType::File(file_info) => retrieve_file_matches(dir_entries, file_info),
        ConfigType::Content(content_info) => retrieve_content_matches(dir_entries, content_info),
        ConfigType::Both {
            file_info,
            content_info,
        } => retrieve_both_matches(dir_entries, file_info, content_info),
    }
}

fn retrieve_file_matches(dir_entries: WalkDir, file_info: &FileInfo) -> Vec<File> {
    dir_entries
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_str()
                .unwrap_or("")
                .contains(&file_info.value)
        })
        .map(|entry| File {
            path: entry.into_path(),
        })
        .collect()
}

fn retrieve_content_matches(dir_entries: WalkDir, content_info: &ContentInfo) -> Vec<File> {
    dir_entries
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            if !entry.file_type().is_file() {
                return false;
            }

            let check = check_file_contains_substring(entry.path(), &content_info.value);

            match check {
                Ok(true) => true,
                Ok(false) => false,
                Err(_) => false,
            }
        })
        .map(|entry| File {
            path: entry.into_path(),
        })
        .collect()
}

fn retrieve_both_matches(dir_entries: WalkDir, file_info: &FileInfo, content_info: &ContentInfo) -> Vec<File> {
    dir_entries
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_str()
                .unwrap_or("")
                .contains(&file_info.value)
        })
        .filter(|entry| {
            if !entry.file_type().is_file() {
                return false;
            }

            let check = check_file_contains_substring(entry.path(), &content_info.value);

            match check {
                Ok(true) => true,
                Ok(false) => false,
                Err(_) => false,
            }
        })
        .map(|entry| File {
            path: entry.into_path(),
        })
        .collect()
}

fn check_file_contains_substring(file: &Path, substring: &str) -> Result<bool, io::Error> {
    let file = fs::File::open(file)?;
    let reader = io::BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;

        if line.contains(substring) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn print_matches(matches: &Vec<File>) {
    let matches_amount = matches.len();

    if matches_amount == 0 {
        println!("No matches found.");
        return;
    } else if matches_amount == 1 {
        println!("Found 1 match:");
    } else {
        println!("Found {} matches:", matches_amount);
    }

    for m in matches {
        println!("{}", m.path.display());
    }
}

fn main() {
    let input = parse_input();
    let config = Config::from_input_args(&input).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    let matches = retrieve_config_matches(&config);
    print_matches(&matches);
}