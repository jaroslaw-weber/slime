use std::fs::File;
use std::io::prelude::*;
use serde_json::Value as SerdeJson;
use handlebars::Handlebars;
use std::error::Error;

pub fn generate_rendered_html(
    rendered_recipe: &str,
    relative_path: &str,
) -> Result<(), Box<Error>> {
    let generated_files_path = "generated";
    let full_path = format!("{}/{}.html", generated_files_path, relative_path);
    let mut file = File::create(full_path)?;
    file.write_all(rendered_recipe.as_bytes())?;
    Ok(())
}

pub fn generate(
    hb: &Handlebars,
    template_name: &str,
    data: &SerdeJson,
    path: &str,
) -> Result<(), Box<Error>> {
    println!("generating: {}", path);
    let rendered = hb.render(template_name, data)?;
    generate_rendered_html(&rendered, path)
}
