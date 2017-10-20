use std::fs::File;
use std::io::prelude::*;
use serde_json;
use serde_json::Value as SerdeJson;
use std::error::Error;

//todo use config file

pub fn read_file(relative_path: &str) -> Result<String, Box<Error>> {
    let mut file = File::open(relative_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
pub fn load_json(data_folder_path: &str) -> Result<SerdeJson, Box<Error>> {
    let data_files_path = "data";
    let path = format!("{}/{}", data_files_path, data_folder_path);
    let as_string = read_file(&path)?;
    let as_json: SerdeJson = serde_json::from_str(&as_string)?;
    Ok(as_json)
}
