use serde_json;
use serde_json::Value as SerdeJson;
use failure::Error;
use toml::Value as Toml;
use fs_extra::file::read_to_string;
use get_filename_with_ext;

/// Load json file from data folder.
pub fn load_json(data_folder_path: &str, file_name: &str) -> Result<SerdeJson, Error> {
    let filename = get_filename_with_ext(file_name, "json");
    let path = format!("{}/{}", data_folder_path, filename);
    let as_string = read_to_string(&path)?;
    let as_json: SerdeJson = serde_json::from_str(&as_string)?;
    Ok(as_json)
}

/// Load toml file from data folder
pub fn load_toml(data_folder_path: &str, file_name: &str) -> Result<Toml, Error> {
    let filename = get_filename_with_ext(file_name, "toml");
    let path = format!("{}/{}", data_folder_path, filename);
    let as_string = read_to_string(&path)?;
    let as_toml: Toml = as_string.parse()?;
    Ok(as_toml)
}
