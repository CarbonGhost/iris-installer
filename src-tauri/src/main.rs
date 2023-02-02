#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod quilt;
mod structs;

use std::{
    fs::{self, create_dir, read_dir, File},
    io::Write,
    path::{Path, PathBuf},
};
use structs::{Meta, ModrinthApi, Version};
use tauri::{Manager, State};
use window_shadows::set_shadow;

use crate::quilt::download_quilt;

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
    let meta = state.client.get("https://gist.githubusercontent.com/CarbonGhost/43899577b696150ad1deee2414967fb8/raw/0bcc88b5ef64d0fccfb6eb7e601ed710578dd5fc/meta.json").send().await;

    match meta {
        Ok(res) => match res.json().await {
            Ok(json) => Ok(json),
            Err(_) => todo!("JSON serialization error"),
        },
        Err(_) => todo!("Networking error"),
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
) -> Result<(), ()> {
    let mod_dir = get_mod_dir(&state.mc_dir, iris, &version.name)?;
    let req = state
        .client
        .get("https://api.modrinth.com/v2/project/iris/version")
        .send()
        .await;

    match req {
        Ok(res) => {
            let iris_jar = res.json().await;
            let req = state
                .client
                .get("https://api.modrinth.com/v2/project/sodium/version")
                .send()
                .await;
            match req {
                Ok(res) => {
                    let sodium = res.json().await;
                    match iris_jar {
                        Ok(iris_jar) => match sodium {
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
                                    iris_jar,
                                    &version.name,
                                    &version.iris_version,
                                    &state.client,
                                    &mod_dir,
                                )
                                .await?;

                                download_quilt(
                                    &state.client,
                                    iris,
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

    Ok(())
}
