extern crate handlebars;
#[macro_use]
extern crate serde_json;
use serde_json::value::Value as SerdeJson;
use std::error::Error;

pub mod template;
pub mod data;
pub mod html;

pub struct Slime<'a> {
	config: Config<'a>,
	pages: Vec<Page<'a>>,
}
struct Page<'a> {
	template_name: &'a str,
	data: SerdeJson,
	generate_path: &'a str,
}

pub enum DataFormat {
	Json,
	//Toml, //todo: support toml
}

impl<'a> Slime<'a> {
	pub fn run(&self) -> Result<(), Box<Error>> {
		let hb = template::load_all().unwrap();
		for p in &self.pages {
			html::generate(
				&hb,
				p.template_name,
				&p.data,
				self.config.generated_files_path,
				p.generate_path,
			)?;
		}
		Ok(())
		//todo: mapping from config file
	}
	pub fn new() -> Slime<'a> {
		Slime::new_with(Config::default())
	}
	pub fn new_with(config: Config<'a>) -> Slime<'a> {
		Slime {
			config: config,
			pages: Vec::new(),
		}
	}
	pub fn add_simple(
		&mut self,
		template_and_data_file_name: &'a str,
		data_format: DataFormat,
	) -> Result<(), Box<Error>> {
		let data = data::load_json(&self.config.data_folder_path, template_and_data_file_name)?;
		let p = Page {
			template_name: template_and_data_file_name,
			data: data,
			generate_path: template_and_data_file_name,
		};
		self.pages.push(p);
		Ok(())
	}
	//more advanced use
	pub fn add(&mut self, template_name: &'a str, data: &SerdeJson, generate_path: &'a str) {
		let p = Page {
			template_name: template_name,
			data: data.clone(), //todo: way to avoid cloning data?
			generate_path: generate_path,
		};
		self.pages.push(p);
	}
	pub fn load_data(
		&self,
		data_path: &str,
		data_format: DataFormat,
	) -> Result<SerdeJson, Box<Error>> {
		//todo: toml
		data::load_json(self.config.data_folder_path, data_path)
	}
}
pub struct Config<'a> {
	data_folder_path: &'a str,
	generated_files_path: &'a str,
	templates_path: &'a str,
}

impl<'a> Config<'a> {
	pub fn default() -> Config<'a> {
		Config {
			data_folder_path: "data",
			generated_files_path: "generated",
			templates_path: "templates",
		}
	}
	pub fn new(data_path: &'a str, generated_path: &'a str, templates_path: &'a str) -> Config<'a> {
		Config {
			data_folder_path: data_path,
			generated_files_path: generated_path,
			templates_path: templates_path,
		}
	}
}
