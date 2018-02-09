use serde_json;
use serde_json::Value as SerdeJson;
use std::error::Error;
use toml::Value as Toml;
use fs_extra::file::*;

//todo use config file

/// Load json file from data folder.
pub fn load_json(data_folder_path: &str, file_name: &str) -> Result<SerdeJson, Box<Error>> {
    let path = format!("{}/{}.json", data_folder_path, file_name);
    let as_string = read_to_string(&path)?;
    let as_json: SerdeJson = serde_json::from_str(&as_string)?;
    Ok(as_json)
}

pub fn load_toml(data_folder_path: &str, file_name: &str) -> Result<Toml, Box<Error>> {
    let path = format!("{}/{}.toml", data_folder_path, file_name);
    let as_string = read_to_string(&path)?;
    let as_toml: Toml = as_string.parse()?;
    Ok(as_toml)
}
