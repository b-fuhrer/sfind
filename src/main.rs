#![allow(unused)]
mod config;

use crate::config::ErrorPolicy;
use clap::{Arg, ArgAction, ArgMatches, command};
use colored::Colorize;
use config::{Config, ConfigType, ContentInfo, FileInfo};
use std::cmp::PartialEq;
use std::error::Error;
use std::ffi::{OsStr};
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::{fs, io, process};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
struct File {
    path: PathBuf,
}

struct MatchResults {
    matches: Vec<File>,
    errors: Vec<io::Error>,
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
        .arg(
            Arg::new("error_policy")
                .short('e')
                .long("error")
                .aliases(["err", "errs", "errors"])
                .required(false)
                .help("Show I/O errors for file content checks.")
                .requires("content")
                .action(ArgAction::SetTrue),
        )
        .get_matches()
}

fn retrieve_config_matches(config: &Config) -> MatchResults {
    let dir_entries = WalkDir::new(&config.starting_directory);
    let initial_results = MatchResults { matches: Vec::new(), errors: Vec::new() };
    let match_results = dir_entries
        .into_iter()
        .filter_map(|entry_result| {
            let entry = entry_result.ok()?;

            if entry.file_type().is_file() {
                return Some(entry);
            }
            None
        })
        .fold(initial_results, |mut current_results, entry|{
            match is_config_match(&entry, config) {
                Ok(true) => current_results.matches.push(File { path: entry.into_path() }),
                Ok(false) => (),
                Err(e) => {
                    if config.error_policy == ErrorPolicy::Display {
                        current_results.errors.push(e)
                    }
                }
            }
            current_results
        });

    match_results
}

fn is_config_match(entry: &DirEntry, config: &Config) -> Result<bool, io::Error> {
    match &config.config_type {
        ConfigType::File(file_info) => Ok(is_file_name_match(entry, &file_info.value)),
        ConfigType::Content(content_info) => is_file_content_match(entry, &content_info.value),
        ConfigType::Both { file_info, content_info } => {
            if is_file_name_match(entry, &file_info.value) {
                return is_file_content_match(entry, &content_info.value);
            }
            Ok(false)
        }
    }
}

fn is_file_name_match(file_entry: &DirEntry, substring: &str) -> bool {
    let file_name = file_entry.file_name();
    file_name
        .to_str()
        .map_or(false, |name| name.contains(substring))
}

fn is_file_content_match(file_entry: &DirEntry, substring: &str) -> Result<bool, io::Error> {
    let file_path = file_entry.path();
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;

        if line.contains(substring) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn print_matches(match_results: &MatchResults) {
    let matches_amount = match_results.matches.len();

    if matches_amount == 0 {
        eprintln!("{}", "No matches found.".red());
        process::exit(1);
    } else if matches_amount == 1 {
        eprintln!("{}", "Found 1 match:".green());
    } else {
        eprintln!("{}", format!("Found {} matches:", matches_amount).green());
    }

    for file in &match_results.matches {
        println!("{}", file.path.display());
    }

    let errors_amount = match_results.errors.len();

    if errors_amount == 0 {
        process::exit(0);
    }

    eprintln!("\n{}", format!("Found {} errors:", errors_amount).red());

    for error in &match_results.errors {
        eprintln!("{}", error);
    }
}

fn main() {
    let input = parse_input();
    let config = Config::from_input_args(&input).unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(-1);
    });
    let match_results = retrieve_config_matches(&config);
    print_matches(&match_results);
}