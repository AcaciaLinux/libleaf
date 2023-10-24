use crate::error::*;
use crate::{config::Config, download, error::LError, package::RemotePackage, util};
use log::{error, info, trace};
use serde::Deserialize;
use std::path::PathBuf;

/// A mirror holding a package list
#[derive(Debug, Deserialize)]
pub struct Mirror {
    /// The name for the mirror
    pub name: String,
    /// The url to fetch the package list from
    pub url: String,

    /// The packages provided by this mirror
    #[serde(skip)]
    pub packages: Option<Vec<RemotePackage>>,
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

            packages: None,
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

        download::download(
            &self.url,
            format!("Updating mirror {}...", self.name).as_str(),
            config.render_bar,
            |data| {
                buf.extend_from_slice(data);
                true
            },
        )?;

        let reader = std::io::Cursor::new(buf);

        let res: serde_json::Value = match serde_json::from_reader(reader) {
            Ok(v) => v,
            Err(e) => return Err(LError::new(LErrorClass::JSON, format!("{}", e).as_str())),
        };

        std::fs::write(self.get_path(config), res["payload"].to_string())?;

        info!("Updated mirror {}", &self.name);

        Ok(())
    }

    /// Loads the mirror data from the saved mirror file
    /// # Arguments
    /// * `config` - A reference to a leaf config struct for getting the mirrors directory
    pub fn load(&mut self, config: &Config) -> Result<(), LError> {
        info!(
            "Loading mirror {} from {}...",
            self.name,
            self.get_path(config).to_string_lossy()
        );

        let data = std::fs::read_to_string(self.get_path(config))?;

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct DE {
            data: Vec<RemotePackage>,
        }

        let buf: DE = match serde_json::from_str(&data) {
            Ok(v) => v,
            Err(e) => {
                return Err(LError::new(
                    LErrorClass::JSON,
                    format!("When loading mirror {}: {}", self.name, e).as_str(),
                ))
            }
        };

        self.packages = Some(buf.data);

        trace!("Packages for mirror {} ({})", self.name, self.url);
        for package in self
            .get_packages()
            .err_prepend("When listing mirror packages:")?
        {
            trace!(
                " - {}-{}-{}",
                package.name,
                package.version,
                package.real_version
            );
        }

        Ok(())
    }

    /// Returns the packages vector or an error that the mirror is not loaded
    pub fn get_packages(&self) -> Result<&Vec<RemotePackage>, LError> {
        match &self.packages {
            Some(p) => Ok(p),
            None => Err(LError::new(
                LErrorClass::MirrorNotLoaded,
                &format!("Mirror {} ({})", self.name, self.url),
            )),
        }
    }
}
