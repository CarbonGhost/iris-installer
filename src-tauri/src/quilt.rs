use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Seek},
    path::Path,
};

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use crate::structs::Version;

async fn get_quilt_version(client: &reqwest::Client) -> Result<String, ()> {
    let req = client
        .get("https://github.com/IrisShaders/Iris-Installer-Maven/blob/master/latest-loader-quilt")
        .send()
        .await;
    match req {
        Ok(res) => {
            let version = res.text().await;
            match version {
                Ok(version) => Ok(version),
                Err(_) => todo!(),
            }
        }
        Err(_) => todo!(),
    }
}

pub async fn download_quilt(
    client: &reqwest::Client,
    iris: bool,
    version: Version,
    generate_profile: bool,
    mc_dir: &Path,
) -> Result<(), ()> {
    let loader_version = get_quilt_version(client).await?;

    if mc_dir.join("launcher_profiles.json").exists() {
        todo!("launcher_profiles.json could not be found")
    }

    let profile_name = if iris {
        todo!()
    } else {
        format!("quilt-loader-{}-{}", loader_version, version.name)
    };

    let profile_dir = mc_dir.join("versions").join(&profile_name);

    if profile_dir.exists() {
        if fs::remove_dir_all(&profile_dir).is_err() {
            todo!()
        };

        todo!()
    }

    if fs::create_dir_all(&profile_dir).is_err() {};

    if File::create(profile_dir.join(profile_name.clone() + ".jar")).is_err() {
        todo!()
    };

    let Ok(mut file) = File::create(profile_dir.join(profile_name.clone() + ".json")) else {
        todo!()
    };

    let Ok(response) = client.get(format!(
        "https://meta.quiltmc.org/v3/versions/loader/{}/{}/profile/json",
        &version.name, &loader_version
    )).send().await else {
        todo!()
    };

    let Ok(mut response) = response.text().await else {
        todo!()
    };

    if iris {
        let mut json: Value = serde_json::from_str(&response).unwrap();

        let args = json
            .as_object_mut()
            .unwrap()
            .get_mut("arguments")
            .unwrap()
            .as_object_mut()
            .unwrap();

        args.insert(
            "jvm".to_string(),
            json!([
                "-Dloader.modsDir=iris-reserved/1.19.3",
                "-Diris.installer=true"
            ]),
        );

        response = serde_json::to_string(&json).unwrap();
    }

    if io::copy(&mut response.as_bytes(), &mut file).is_err() {
        todo!()
    };

    if generate_profile {
        let Ok(mut file) = fs::OpenOptions::new().read(true).write(true).open(
            mc_dir
                .join("launcher_profiles")
                .with_extension("json"),
        ) else {
            todo!()
        };

        let Ok(mut launcher_profiles): Result<LauncherProfiles, serde_json::Error> = serde_json::from_reader(&file) else {
            todo!()
        };
        if file.set_len(0).is_err() {
            todo!()
        };
        if file.rewind().is_err() {
            todo!()
        };

        launcher_profiles.profiles.insert(
            profile_name.clone(),
            Profile {
                name: if iris {
                    format!("Iris {}", &version.name)
                } else {
                    format!("Quilt Loader {}", &version.name)
                },
                profile_type: "custom".into(),
                created: Utc::now(),
                last_version_id: profile_name,
                icon: format!("data:image/png;base64,{}", BASE64.encode(get_icon(iris))),
                other: Map::new(),
            },
        );

        if serde_json::to_writer_pretty(file, &launcher_profiles).is_err() {
            todo!()
        };
    }

    Ok(())
}

const QUILT_ICON: &[u8] = include_bytes!("quilt.png");

const IRIS_ICON: &[u8] = include_bytes!("iris.png");

fn get_icon(iris: bool) -> &'static [u8] {
    if iris {
        IRIS_ICON
    } else {
        QUILT_ICON
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LauncherProfiles {
    profiles: HashMap<String, Profile>,
    #[serde(flatten)]
    other: Map<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Profile {
    name: String,
    #[serde(rename = "type")]
    profile_type: String,
    created: DateTime<Utc>,
    last_version_id: String,
    icon: String,
    #[serde(flatten)]
    other: Map<String, Value>,
}
