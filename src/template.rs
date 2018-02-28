use std::fs::read_dir;
use handlebars::Handlebars;
use std::path::Path;
use failure::Error;
use Config;

/// Loading all templates from template folder
pub fn load_all(config: &Config) -> Result<Handlebars, Error> {
    //todo: load templates only once
    let mut handlebars = Handlebars::new();
    register_all_templates(config, &mut handlebars)?;
    Ok(handlebars)
}

/// register all templates from template folder to one Handlebars object
fn register_all_templates(config: &Config, handlebars: &mut Handlebars) -> Result<(), Error> {
    let templates_path = config.templates_path;
    let paths = read_dir(templates_path)?;

    for path in paths {
        let uw = path?;
        let relative = uw.path().display().to_string(); //example: ./templates/category.hbs
        if relative.ends_with(".hbs") {
            println!("registering template: {}", &relative);
            let filename_option = path_to_filename(&relative);
            match filename_option
            {
                Some(filename) => handlebars.register_template_file(&filename, &Path::new(&relative))?,
                None => bail!("fail to get filename")
            }
            
        }
    }
    Ok(())
}

/// Get filename from path
/// Example:
/// ./folder/filename.ext -> filename
fn path_to_filename(relative_path: &str) -> Option<String> {
    
    let path = relative_path
        .split('/')
        .last()?
        .split('.')
        .next()?
        .to_string();
        
    Some(path)
}
