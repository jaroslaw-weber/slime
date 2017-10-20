extern crate slime;
use slime::data::load_json;
use slime::html::generate;
use slime::template::load_all;

fn main() {
    //load all templates in template folder
    let hb = load_all().expect("failed to get templates");
    //load data from relative path in data folder
    let index_data = load_json("index.json").expect("failed to get index data");
    //generate html file. arguments: handlebars, template name, data, generated file relative path
    generate(&hb, "index", &index_data, "index").expect("failed to generate html");
}




