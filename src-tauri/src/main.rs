#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod structs;

use std::{
    collections::HashMap,
    fs::{self, create_dir, read_dir, File},
    io::{self, Seek, Write},
    path::{Path, PathBuf},
};

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use structs::{Meta, ModrinthApi, Version};
use tauri::{Manager, State};
use window_shadows::set_shadow;

struct AppState {
    client: reqwest::Client,
    mc_dir: PathBuf,
}

impl AppState {
    fn new() -> Self {
        let client = reqwest::Client::new();
        let mc_dir = get_default_client_dir();
        Self { client, mc_dir }
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::new())
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            set_shadow(&window, true).expect("Unsupported platform!");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            fetch_meta,
            versions,
            download_mods,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn fetch_meta(state: State<'_, AppState>) -> Result<Meta, ()> {
    let meta = state.client.get("https://raw.githubusercontent.com/IrisShaders/Iris-Installer-Files/master/meta-new.json").send().await;
    match meta {
        Ok(res) => match res.json().await {
            Ok(json) => Ok(json),
            Err(_) => todo!(),
        },
        Err(_) => todo!(),
    }
}

#[tauri::command]
async fn versions(outdated: bool, snapshot: bool, meta: Meta) -> Result<Vec<Version>, ()> {
    let mut versions = Vec::new();
    for version in meta.versions {
        if snapshot == version.snapshot || outdated == version.outdated {
            versions.push(version.clone());
        } else if !version.outdated && !version.snapshot {
            versions.push(version.clone())
        }
    }
    Ok(versions)
}

#[cfg(target_os = "windows")]
const HOMEDIR: &str = "APPDATA";

#[cfg(any(target_os = "macos", target_os = "linux"))]
const HOMEDIR: &str = "HOME";

fn get_default_client_dir() -> PathBuf {
    let base_dir = PathBuf::from(std::env::var(HOMEDIR).unwrap());
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    return base_dir.join(".minecraft");
    #[cfg(target_os = "macos")]
    return base_dir
        .join("Library")
        .join("Application Support")
        .join("minecraft");
}

fn get_mod_dir(mc_dir: &Path, iris: bool, version: &String) -> Result<PathBuf, ()> {
    let mod_dir = if iris {
        mc_dir.join("iris-reservec").join(version)
    } else {
        mc_dir.join("mods")
    };
    if mod_dir.exists() {
        Ok(mod_dir)
    } else {
        match create_dir(mod_dir.clone()) {
            Ok(_) => Ok(mod_dir),
            Err(err) => panic!("{}", err),
        }
    }
}

#[tauri::command]
async fn download_mods(
    iris: bool,
    state: State<'_, AppState>,
    version: Version,
    generate_profile: bool,
    quilt: bool,
) -> Result<(), ()> {
    let mod_dir = get_mod_dir(&state.mc_dir, iris, &version.name)?;
    let req = state
        .client
        .get("https://api.modrinth.com/v2/project/iris/version")
        .send()
        .await;
    match req {
        Ok(res) => {
            let iris = res.json().await;
            let req = state
                .client
                .get("https://api.modrinth.com/v2/project/sodium/version")
                .send()
                .await;
            match req {
                Ok(res) => {
                    let sodium = res.json().await;
                    match iris {
                        Ok(iris) => match sodium {
                            Ok(sodium) => {
                                jar_writer(
                                    sodium,
                                    &version.name,
                                    &version.sodium_version,
                                    &state.client,
                                    &mod_dir,
                                )
                                .await?;
                                jar_writer(
                                    iris,
                                    &version.name,
                                    &version.iris_version,
                                    &state.client,
                                    &mod_dir,
                                )
                                .await?;

                                download_quilt(
                                    &state.client,
                                    quilt,
                                    version,
                                    generate_profile,
                                    &state.mc_dir,
                                )
                                .await?;

                                Ok(())
                            }
                            Err(_err) => todo!(),
                        },
                        Err(_err) => todo!(),
                    }
                }
                Err(_err) => todo!(),
            }
        }
        Err(_err) => todo!(),
    }
}

async fn jar_writer(
    data: Vec<ModrinthApi>,
    version: &String,
    mod_version: &String,
    client: &reqwest::Client,
    mod_dir: &Path,
) -> Result<(), ()> {
    for details in data {
        if details.game_versions.contains(version) && details.version_number.contains(mod_version) {
            let url = &details.files[0].url;
            let Ok(req) = client.get(url).send().await else {
                todo!()
            };
            let Ok(buf) = req.bytes().await else {
                todo!()
            };
            println!("{:?}", mod_dir.join(details.files[0].filename.clone()));
            let Ok(mut file) = File::create(mod_dir.join(details.files[0].filename.clone())) else {
                // cannot make jar
                todo!()
            };
            let Ok(_) = file.write(&buf) else {
                // cannot write to just made jar??
                todo!()
            };
            break;
        }
    }
    let Ok(jars) = read_dir(mod_dir) else {
        todo!("Cannot Access Folder")
    };
    for jar in jars {
        match jar {
            Ok(jar) => {
                let name = jar.file_name().into_string().unwrap();
                if name.contains("iris") || name.contains("sodium") {
                    let Ok(_) = fs::remove_file(jar.path()) else {
                        todo!()
                    };
                }
            }
            Err(_) => todo!(),
        }
    }
    Ok(())
}

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

async fn download_quilt(
    client: &reqwest::Client,
    iris: bool,
    version: Version,
    generate_profile: bool,
    mc_dir: &Path,
) -> Result<(), ()> {
    let loader_version = get_quilt_version(client).await?;

    if mc_dir.join("launcher_profiles.json").exists() {
        todo!()
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

    if iris {
        let Ok(response) = &mut response.text().await else {
            todo!()
        };
    
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

        response = &mut serde_json::to_string(&json).unwrap();
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
