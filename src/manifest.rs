use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub enum ManifestPlatform {
    #[serde(rename = "windows")]
    Windows,
    #[serde(rename = "linux")]
    Linux,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<ManifestPlatform>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ManifestFile {
    pub path: String,
    pub hash: String,
    pub size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Metadata>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ManifestPatches {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<String>,
    pub files: Vec<ManifestFile>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Manifest {
    pub version: String,
    pub patches: Vec<ManifestPatches>,
}

impl Manifest {}
