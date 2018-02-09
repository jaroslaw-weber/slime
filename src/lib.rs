extern crate fs_extra;
extern crate handlebars;
extern crate serde;
extern crate serde_json;
extern crate toml;
use serde_json::value::Value as SerdeJson;
use std::error::Error;
use toml::Value as Toml;
use serde::ser::Serialize;

pub mod template;
pub mod data;
pub mod html;

/// Rust library for fast prototyping and creating static websites.
/// Use handlebars to handle html parts.
/// Uses json and toml formats for data passed into templates.
/// Flexible design allows to create any type of static website.

/// Creating new project:
/// Create new binary crate with
/// ```
/// cargo new --bin crate_name
/// ```
/// Open crate folder and run
/// curl -s https://raw.githubusercontent.com/jaroslaw-weber/slime/master/init_slime.sh | bash
/// This will download deployment scripts and create some folders.
/// It will also create example files.

/// Wrapper for creating a static websites.
/// Load templates and helps with loading and inserting data into templates.
pub struct Slime<'a> {
    config: Config<'a>,
}

impl<'a> Slime<'a> {
    /// Create new wrapper with custom config.
    pub fn new(config: Config<'a>) -> Slime<'a> {
        Slime { config: config }
    }

    /// Load json data from data folder.
    /// Data file needs to has ".json" extension suffix!
    /// Filename passed as argument needs to be without extension
    pub fn load_json_data(&self, file_name: &str) -> Result<SerdeJson, Box<Error>> {
        data::load_json(self.config.data_path, file_name)
    }

    /// Load toml data from data folder. Same rules apply as in load_json_data.
    pub fn load_toml_data(&self, file_name: &str) -> Result<Toml, Box<Error>> {
        data::load_toml(self.config.data_path, file_name)
    }

    /// Generate html page.
    pub fn generate<T: Serialize>(
        &self,
        template: &str,
        file_name: &str,
        data: &T,
    ) -> Result<(), Box<Error>> {
        let hb = template::load_all(&self.config).unwrap();
        html::generate(&hb, template, data, self.config.generated_path, file_name)?;
        Ok(())
    }
}

impl<'a> Default for Slime<'a> {
    fn default() -> Slime<'a> {
        Slime::new(Config::default())
    }
}

/// Folders paths config file.
pub struct Config<'a> {
    data_path: &'a str,
    generated_path: &'a str,
    templates_path: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(data_path: &'a str, generated_path: &'a str, templates_path: &'a str) -> Config<'a> {
        Config {
            data_path: data_path,
            generated_path: generated_path,
            templates_path: templates_path,
        }
    }
}

impl<'a> Default for Config<'a> {
    fn default() -> Config<'a> {
        Config {
            data_path: "data",
            generated_path: "generated",
            templates_path: "templates",
        }
    }
}
