extern crate slime;
#[macro_use]
extern crate serde_json;
use slime::Slime;
use slime::DataFormat;


fn main() {
    //create new Slime wrapper
    let mut s = Slime::new();
    //link "index.hbs" with "index.json"
    s.add_simple("index", DataFormat::Json).expect("failed to add page");
    let mut some_data = s.load_data("index", DataFormat::Json).expect("failed to load json data");
    //manipulate loaded data
    some_data["test"]=json!("changed");
    //link "index.hbs" with some manipulated data and generate "index2.html" (only save settings)
    s.add("index", &some_data, "index2");
    //try to generate a website
    s.run().expect("failed to generate website");
}




