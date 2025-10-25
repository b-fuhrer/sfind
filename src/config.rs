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

#[derive(PartialEq, Debug)]
pub enum ErrorPolicy {
    Display,
    Ignore,
}

#[derive(Debug)]
pub struct Config {
    pub starting_directory: PathBuf,
    pub config_type: ConfigType,
    pub error_policy: ErrorPolicy,
}

impl Config {
    pub fn from_input_args(input: &ArgMatches) -> Result<Config, String> {
        let starting_directory = PathBuf::from(input.get_one::<String>("start_dir").cloned().unwrap());

        if !starting_directory.is_dir() {
            return Err(format!("{} is not a valid directory.", starting_directory.display()));
        }

        let file = input.get_one::<String>("file_name").cloned();
        let content = input.get_one::<String>("content").cloned();

        let error_policy = if input.get_flag("error_policy") {
            ErrorPolicy::Display
        } else {
            ErrorPolicy::Ignore
        };

        Config::new(starting_directory, file, content, error_policy)
    }

    pub fn new(
        starting_directory: PathBuf, file: Option<String>, content: Option<String>, error_policy: ErrorPolicy,
    ) -> Result<Config, String> {
        match (file, content) {
            (Some(f), Some(c)) => Ok(Config {
                starting_directory,
                config_type: ConfigType::Both {
                    file_info: FileInfo { value: f },
                    content_info: ContentInfo { value: c },
                },
                error_policy,
            }),
            (Some(f), None) => {
                Ok(Config { starting_directory, config_type: ConfigType::File(FileInfo { value: f }), error_policy })
            }
            (None, Some(c)) => Ok(Config {
                starting_directory,
                config_type: ConfigType::Content(ContentInfo { value: c }),
                error_policy,
            }),
            (None, None) => Err(String::from(
                "Both file name and content are missing. Please provide at least one!",
            )),
        }
    }
}
