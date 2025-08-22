use clap::ArgMatches;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileInfo {
    pub value: String,
}

#[derive(Debug)]
pub struct ContentInfo {
    pub value: String,
}

#[derive(Debug)]
pub enum ConfigType {
    File(FileInfo),
    Content(ContentInfo),
    Both {
        file_info: FileInfo,
        content_info: ContentInfo,
    },
}

#[derive(Debug)]
pub struct Config {
    pub starting_directory: PathBuf,
    pub config_type: ConfigType,
}

impl Config {
    pub fn from_input_args(input: &ArgMatches) -> Result<Config, String> {
        let starting_directory =
            PathBuf::from(input.get_one::<String>("start_dir").cloned().unwrap());

        if !starting_directory.is_dir() {
            return Err(format!(
                "{} is not a valid directory.",
                starting_directory.display()
            ));
        }

        let file = input.get_one::<String>("file_name").cloned();
        let content = input.get_one::<String>("content").cloned();

        Config::new(starting_directory, file, content)
    }

    pub fn new(
        starting_directory: PathBuf,
        file: Option<String>,
        content: Option<String>,
    ) -> Result<Config, String> {
        match (file, content) {
            (Some(f), Some(c)) => Ok(Config {
                starting_directory,
                config_type: ConfigType::Both {
                    file_info: FileInfo { value: f },
                    content_info: ContentInfo { value: c },
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
            (None, None) => Err(String::from(
                "Both file name and content are missing. Please provide at least one!",
            )),
        }
    }

    pub fn get_file_info(&self) -> Option<&FileInfo> {
        match &self.config_type {
            ConfigType::File(f) => Some(&f),
            ConfigType::Content(_) => None,
            ConfigType::Both { file_info: f, .. } => Some(&f),
        }
    }

    pub fn get_content_info(&self) -> Option<&ContentInfo> {
        match &self.config_type {
            ConfigType::File(_) => None,
            ConfigType::Content(c) => Some(&c),
            ConfigType::Both {
                content_info: c, ..
            } => Some(&c),
        }
    }
}