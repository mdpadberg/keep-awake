use anyhow::{Result, Context};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub application_name: String,
}

fn get_path() -> Option<(PathBuf, PathBuf)> {
    let mut folders = config_dir()?;
    folders.push("ka");
    let filename: String = String::from("settings.json");
    Some((folders.clone(), folders.join(&filename)))
}

impl Settings {
    pub fn load() -> Option<Settings> {
        if let Ok(settings_file_as_string) = fs::read_to_string(get_path()?.1) {
            serde_json::from_str(&settings_file_as_string).ok()
        } else {
            None
        }
    }

    pub fn save(self) -> Result<()> {
        let path = get_path().context("no path to settings file")?;
        fs::create_dir_all(path.0)?;
        let mut file = File::create(path.1)?;
        Ok(file.write_all(serde_json::to_string(&self)?.as_bytes())?)
    }
}
