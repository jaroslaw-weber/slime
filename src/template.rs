use std::fs::read_dir;
use handlebars::Handlebars;
use std::path::Path;
use std::error::Error;

pub fn load_all() -> Result<Handlebars, Box<Error>> {
    let mut handlebars = Handlebars::new();
    register_all_templates(&mut handlebars)?;
    Ok(handlebars)
}

fn register_all_templates(handlebars: &mut Handlebars) -> Result<(), Box<Error>> {
    let templates_path = "templates";
    let paths = read_dir(templates_path)?;

    for path in paths {
        let uw = path?;
        let relative = uw.path().display().to_string(); //example: ./templates/category.hbs
        if relative.ends_with(".hbs") {
            println!("registering template: {}", &relative);
            let filename = path_to_filename(&relative)?;
            handlebars.register_template_file(&filename, &Path::new(&relative))?;
        }
    }
    Ok(())
}

// ./folder/filename.ext -> filename
fn path_to_filename(relative_path: &str) -> Result<String, Box<Error>> {
    let path = relative_path
        .split("/")
        .last()
        .ok_or("failed to get filename")?
        .split(".")
        .next()
        .ok_or("failed to get filename")?
        .to_string();
    Ok(path)
}
