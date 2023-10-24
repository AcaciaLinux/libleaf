use serde::Deserialize;

/// A remote package, ready to be fetched from a remote server
#[derive(Debug, Deserialize)]
pub struct RemotePackage {
    pub name: String,
    pub version: String,
    #[serde(deserialize_with = "crate::util::deserialize_number_from_string")]
    pub real_version: u64,
    pub description: String,
    pub dependencies: Vec<String>,
    pub hash: String,
    pub url: String,
}
