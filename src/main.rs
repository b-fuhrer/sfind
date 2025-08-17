use std::path::PathBuf;
use std::process;
use clap::{command, Arg, ArgMatches};

#[derive(Debug)]
struct FileInfo {
    value: String,
}

#[derive(Debug)]
struct ContentInfo {
    value: String,
}

#[derive(Debug)]
enum ConfigType {
    File(FileInfo),
    Content(ContentInfo),
    Both {file_info: FileInfo, content_info: ContentInfo},
}

#[derive(Debug)]
struct Config {
    starting_directory: PathBuf,
    config_type: ConfigType,
}

impl Config {
    pub fn from_input_args(input: &ArgMatches) -> Result<Config, String> {
        let starting_directory = PathBuf::from(
            input.get_one::<String>("start_dir").cloned().unwrap()
        );

        if !starting_directory.is_dir() {
            return Err(format!("{} is not a valid directory.", starting_directory.display()));
        }

        let file = input.get_one::<String>("file_name").cloned();
        let content = input.get_one::<String>("content").cloned();

        Config::new(starting_directory, file, content)
    }

    pub fn new(starting_directory: PathBuf, file: Option<String>, content: Option<String>) -> Result<Config, String> {
        match (file, content) {
            (Some(f), Some(c)) => Ok(Config {
                starting_directory,
                config_type: ConfigType::Both {
                    file_info: FileInfo {value: f},
                    content_info: ContentInfo {value: c}
                },
            }),
            (Some(f), None) => Ok(Config {
                starting_directory,
                config_type: ConfigType::File(FileInfo { value: f }),
            }),
            (None, Some(c)) => Ok(Config {
                starting_directory,
                config_type: ConfigType::Content(ContentInfo { value: c }),
            }),
            (None, None) => Err(String::from("Both file name and content are missing. Please provide at least one!")),
        }
    }

    pub fn get_file_info(&self) -> Option<&FileInfo> {
        match &self.config_type {
            ConfigType::File(f) => Some(&f),
            ConfigType::Content(_) => None,
            ConfigType::Both { file_info: f, .. } => Some(&f)
        }
    }

    pub fn get_content_info(&self) -> Option<&ContentInfo> {
        match &self.config_type {
            ConfigType::File(_) => None,
            ConfigType::Content(c) => Some(&c),
            ConfigType::Both { content_info: c, .. } => Some(&c)
        }
    }
}

fn parse_input() -> ArgMatches {
    command!()
        .arg(Arg::new("start_dir")
            .short('d')
            .long("dir")
            .aliases(["directory", "start", "startdir", "startdirectory", "start-dir", "start-directory", "sd", "sdir"])
            .required(false)
            .help("Start directory where the find should start its recursive descent.")
            .default_value(".")
        )
        .arg(Arg::new("file_name")
            .short('f')
            .long("file")
            .aliases(["filename", "fn"])
            .required(false)
            .help("Substring that must be contained in the file name searched for.")
        )
        .arg(Arg::new("content")
            .short('c')
            .long("content")
            .required(false)
            .help("Substring that must be contained in the contents of the file.")
        )
        .get_matches()
}

fn main() {
    let input = parse_input();
    let config = Config::from_input_args(&input).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    println!("{:#?}", config);
}
