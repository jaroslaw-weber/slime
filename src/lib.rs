#[macro_use]
extern crate failure;
extern crate fs_extra;
extern crate handlebars;
extern crate serde;
extern crate serde_json;
extern crate toml;

use serde_json::value::Value as SerdeJson;
use toml::Value as Toml;
use serde::ser::Serialize;
use handlebars::Handlebars;
use failure::Error;

pub mod template;
pub mod data;
pub mod generate;

/// Rust library for fast prototyping and creating static websites.
/// Use handlebars to handle html parts.
/// Uses json and toml formats for data passed into templates.
/// Flexible design allows to create any type of static website.

/// Creating new project:
/// Create new binary crate with
/// ```ignore
/// cargo new --bin crate_name
/// ```
/// Open crate folder and run
/// ```ignore
/// curl -s https://raw.githubusercontent.com/jaroslaw-weber/slime/master/init_slime.sh | bash
/// ```
/// This will download deployment scripts and create some folders.
/// It will also create example files.

/// Wrapper for creating a static websites.
/// Load templates and helps with loading and inserting data into templates.
pub struct Slime<'a> {
    /// cached config
    config: Config<'a>,
    /// cached templates
    templates: Option<Handlebars>,
}

impl<'a> Slime<'a> {
    /// Create new wrapper with custom config.
    pub fn new(config: Config<'a>) -> Slime<'a> {
        Slime {
            config: config,
            templates: None,
        }
    }

    /// Load templates (need to run only once)
    pub fn initialize(&mut self) -> Result<(), Error> {
        let hb: Handlebars = template::load_all(&self.config)?;
        self.templates = Some(hb);
        Ok(())
    }

    /// Load json data from data folder.
    /// Default file extension is ".json" if not specified.
    pub fn load_json_data(&self, file_name: &str) -> Result<SerdeJson, Error> {
        data::load_json(self.config.data_path, file_name)
    }

    /// Load toml data from data folder.
    /// Default file extension is ".toml" if not specified.
    pub fn load_toml_data(&self, file_name: &str) -> Result<Toml, Error> {
        data::load_toml(self.config.data_path, file_name)
    }

    /// Generate file. If filename has no extension then default extension is added (.html)
    pub fn generate<T: Serialize>(
        &self,
        template: &str,
        file_name: &str,
        data: &T,
    ) -> Result<(), Error> {
        match &self.templates {
            &Some(ref hb) => {
                generate::generate(&hb, template, data, self.config.generated_path, file_name)?;
                Ok(())
            }
            &None => bail!("templates not loaded! use Slime::initialize()"),
        }
    }

    /// Copy files from "static" folder to "generated" folder
    /// Using for copying javascript/image files after generation.
    pub fn copy_static_files(&self) -> Result<(), Error> {
        let options = fs_extra::dir::CopyOptions::new();
        fs_extra::dir::copy(self.config.static_path, self.config.generated_path, &options)?;
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
    static_path: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(
        data_path: &'a str,
        generated_path: &'a str,
        templates_path: &'a str,
        static_path: &'a str,
    ) -> Config<'a> {
        Config {
            data_path: data_path,
            generated_path: generated_path,
            templates_path: templates_path,
            static_path: static_path,
        }
    }
}

/// Default folder paths (working with installation script)
impl<'a> Default for Config<'a> {
    fn default() -> Config<'a> {
        Config {
            data_path: "data",
            generated_path: "generated",
            templates_path: "templates",
            static_path: "static",
        }
    }
}

/// does filename has extension in it?
fn has_ext(file_name: &str) -> bool {
    file_name.contains(".")
}

fn get_filename_with_ext(file_name: &str, extension: &str) -> String {
    match has_ext(file_name) {
        true => file_name.to_string(),
        false => format!("{}.{}", file_name, extension),
    }
}
