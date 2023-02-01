use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub beta_version_snippet: String,
    pub has_beta: bool,
    pub versions: Vec<Version>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub name: String,
    pub iris_version: String,
    pub sodium_version: String,
    pub outdated: bool,
    pub snapshot: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModrinthApi {
    pub id: String,
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(rename = "author_id")]
    pub author_id: String,
    pub featured: bool,
    pub name: String,
    #[serde(rename = "version_number")]
    pub version_number: String,
    pub changelog: String,
    #[serde(rename = "changelog_url")]
    pub changelog_url: Option<String>,
    #[serde(rename = "date_published")]
    pub date_published: String,
    pub downloads: i64,
    #[serde(rename = "version_type")]
    pub version_type: String,
    pub status: String,
    #[serde(rename = "requested_status")]
    pub requested_status: Option<String>,
    pub files: Vec<JarFile>,
    pub dependencies: Vec<Dependency>,
    #[serde(rename = "game_versions")]
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JarFile {
    pub hashes: Hashes,
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: i64,
    #[serde(rename = "file_type")]
    pub file_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hashes {
    pub sha1: String,
    pub sha512: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    #[serde(rename = "version_id")]
    pub version_id: Option<String>,
    #[serde(rename = "project_id")]
    pub project_id: Option<String>,
    #[serde(rename = "file_name")]
    pub file_name: Option<String>,
    #[serde(rename = "dependency_type")]
    pub dependency_type: String,
}
