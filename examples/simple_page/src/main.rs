extern crate slime;
use slime::Slime;

fn main() {
    //create new wrapper
    let mut s = Slime::default();

    //load templates
    s.initialize().expect("failed to initialize slime");

    //load data
    let data1 = s.load_json_data("data1").expect("failed to load json data");
    let data2 = s.load_toml_data("data2").expect("failed to load toml data");

    //generate html files
    s.generate("index", "json_version", &data1)
        .expect("failed to generate page with json data");
    s.generate("index", "toml_version", &data2)
        .expect("failed to generate page with toml data");
}
