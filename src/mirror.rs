use crate::{config::Config, download, error::LError, package::RemotePackage, util};
use log::{error, info};
use serde::Deserialize;
use std::path::PathBuf;
use crate::error::*;

/// A mirror holding a package list
#[derive(Debug, Deserialize)]
pub struct Mirror {
    /// The name for the mirror
    pub name: String,
    /// The url to fetch the package list from
    pub url: String,

    /// The packages provided by this mirror
    #[serde(skip)]
    pub packages: Vec<RemotePackage>,
}

impl Mirror {
    /// Creates a new mirror
    /// # Arguments
    /// * `name` - The name for the mirror
    /// * `url` - The url for the package list
    pub fn new(name: &str, url: &str) -> Self {
        Self {
            name: name.to_owned(),
            url: url.to_owned(),

            packages: Vec::new(),
        }
    }

    /// Returns the path the mirror should be stored in
    /// # Arguments
    /// * `config` - A reference to a leaf config struct for getting the mirrors directory
    pub fn get_path(&self, config: &Config) -> PathBuf {
        config
            .get_mirrors_dir()
            .join(self.name.to_owned() + ".json")
    }

    /// Updates the local copy of the mirror's package list
    /// # Arguments
    /// * `config` - The config to use
    pub fn update(&self, config: &Config) -> Result<(), LError> {
        util::ensure_dir(&config.get_mirrors_dir())?;

        let mut buf: Vec<u8> = Vec::new();

        match download::download(
            &self.url,
            format!("Updating mirror {}...", self.name).as_str(),
            config.render_bar,
            |data| {
                buf.extend_from_slice(data);
                true
            },
        ) {
            Ok(_) => (),
            Err(e) => {
                error!(
                    "Failed to update mirror {}: {}",
                    &self.name,
                    e
                );
                return Err(e);
            }
        }

        let reader = std::io::Cursor::new(buf);

        let res: serde_json::Value = match serde_json::from_reader(reader) {
            Ok(v) => v,
            Err(e) => {
                return Err(LError::new(
                    LErrorClass::JSON,
                    format!("When updating mirror {}: {}", self.name, e).as_str(),
                ))
            }
        };

        std::fs::write(self.get_path(config), res["payload"].to_string())?;

        info!("Updated mirror {}", &self.name);

        Ok(())
    }
}
