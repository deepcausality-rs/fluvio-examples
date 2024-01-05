use config_file::FromConfigFile;
use serde::Deserialize;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct ConfigFile {
    data_folder: String,
}

impl ConfigFile {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let config = ConfigFile::from_config_file(path).expect("Failed to parse config file");

        let data_folder = config.data_folder;

        Ok(Self { data_folder })
    }
}

impl ConfigFile {
    pub fn data_folder(&self) -> &str {
        &self.data_folder
    }
}

impl Display for ConfigFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "data_folder: {}", self.data_folder)
    }
}
