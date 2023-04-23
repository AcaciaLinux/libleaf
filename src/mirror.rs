use crate::download;
use crate::{config::Config, usererr, usermsg};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::error::*;
use crate::package::remote::RemotePackage;

/// Represents an online mirror leaf can query package lists from to provide packages
#[derive(Debug)]
pub struct Mirror {
    pub name: String,
    pub url: String,
    pub packages: Option<Vec<RemotePackage>>,
}

impl Mirror {
    /// Creates a new mirror
    /// # Arguments
    /// * `name` - The internal name for the mirror
    /// * `url` - The url leaf can consolidate for fetching the package list
    pub fn new(name: &str, url: &str) -> Mirror {
        Mirror {
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

    /// Queries the mirrors url for fetching the latest package list
    /// # Arguments
    /// * `config` - A reference to a leaf config struct for getting the save path and behaviour information
    pub fn update(&self, config: &Config) -> Result<(), LError> {
        crate::util::ensure_dirs(config)?;

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
                usererr!(
                    "Failed to update mirror {}: {}",
                    &self.name,
                    e.clone().message.unwrap_or("".to_string())
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
                    format!("When updating mirror {}: {}", self.name, e.to_string()).as_str(),
                ))
            }
        };

        std::fs::write(self.get_path(config), res["payload"].to_string())?;

        usermsg!("Updated mirror {}", &self.name);

        Ok(())
    }

    /// Loads the mirror data from the saved mirror file
    /// # Arguments
    /// * `config` - A reference to a leaf config struct for getting the mirrors directory
    pub fn load(&mut self, config: &Config) -> Result<(), LError> {
        info!(
            "Loading mirror {} from {}...",
            self.name,
            self.get_path(config).to_str().unwrap_or("")
        );

        let data = std::fs::read_to_string(self.get_path(config))?;
        self.packages = match serde_json::from_str(&data) {
            Ok(v) => v,
            Err(e) => {
                return Err(LError::new(
                    LErrorClass::JSON,
                    format!("When loading mirror {}: {}", self.name, e.to_string()).as_str(),
                ))
            }
        };

        for package in &self.packages {
            trace!("Mirror {} has {:?}", self.name, package);
        }

        Ok(())
    }

    /// Searches this mirror for a package with the supplied name
    /// # Arguments
    /// * `name` - The package name to search for
    /// # Returns
    /// A reference to the package
    pub fn find_package<'a>(&'a self, name: &str) -> Result<&'a RemotePackage, LError> {
        match &self.packages {
            None => Err(LError::new(LErrorClass::MirrorNotLoaded, &self.name)),
            Some(p) => match crate::util::find_package(name, &p) {
                None => Err(LError::new(LErrorClass::PackageNotFound, name)),
                Some(p) => Ok(p),
            },
        }
    }
}

/// Searches the provided mirrors for a package with the supplied name.
/// # Arguments
/// * `name` - The package name to search for
/// * `mirrors` - The mirrors to search in
/// # Returns
/// A clone of the package
pub fn resolve_package(
    name: &str,
    mirrors: &Vec<Arc<Mutex<Mirror>>>,
) -> Result<RemotePackage, LError> {
    for mirror in mirrors {
        let mirror = mirror.lock().expect("Lock mirror mutex");

        match mirror.find_package(name) {
            Ok(p) => {
                debug!("Mirror {} has package {}", mirror.name, name);
                return Ok(p.clone());
            }
            Err(e) => {
                if e.class == LErrorClass::PackageNotFound {
                    continue;
                }

                return Err(e);
            }
        }
    }

    Err(LError::new(LErrorClass::PackageNotFound, name))
}
