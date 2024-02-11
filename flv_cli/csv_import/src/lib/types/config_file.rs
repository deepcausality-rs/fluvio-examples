use config_file::FromConfigFile;
use serde::Deserialize;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Represents the application configuration file.
///
/// # Fields
///
/// `data_folder` - The root folder for data files
///
/// # Methods
///
/// `from_file` - Creates a ConfigFile from the given file path
///
/// `data_folder` - Getter for the data_folder field
///
/// # Implements
///
/// `Deserialize` - Deserialize from TOML
///
/// `Display` - Display the config as a string
#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct ConfigFile {
    data_folder: String,
    parallel: bool,
}

impl ConfigFile {
    /// Creates a ConfigFile from the given file path.
    ///
    /// Loads the config file using `from_config_file()`.
    ///
    /// Extracts the `data_folder` field.
    ///
    /// Returns a new ConfigFile instance.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the config file
    ///
    /// # Returns
    ///
    /// `Result<Self, Box<dyn Error>>` - Ok with ConfigFile or Err on failure
    ///
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let config =
            ConfigFile::from_config_file(path).expect("ConfigFile: Failed to parse config file");

        let data_folder = config.data_folder;
        let parallel = config.parallel;

        Ok(Self {
            data_folder,
            parallel,
        })
    }
}

impl ConfigFile {
    /// Gets a reference to the data_folder field.
    ///
    /// # Returns
    ///
    /// &str - A reference to the data_folder field
    ///
    pub fn data_folder(&self) -> &str {
        &self.data_folder
    }

    /// Gets a reference to the parallel field.
    ///
    /// # Returns
    ///
    /// bool - true when parallel is enabled, false otherwise
    pub fn parallel(&self) -> bool {
        self.parallel
    }
}

impl Display for ConfigFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "data_folder: {}\n parallel: {}",
            self.data_folder, self.parallel
        )
    }
}
