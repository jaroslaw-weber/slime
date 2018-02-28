use handlebars::Handlebars;
use failure::Error;
use fs_extra::file::*;
use serde::ser::Serialize;
use get_filename_with_ext;

/// Write html file
pub fn generate_rendered_template(
    content: &str,
    folder_path: &str,
    file_name: &str,
) -> Result<(), Error> {
    let full_path = format!("{}/{}", folder_path, file_name);
    //todo generate folder if not existing (solving github ignore problem)
    write_all(full_path, content)?;
    Ok(())
}

//todo set default extension for files in config!

/// Generate file. 
/// If filename doesn't have extension specified, then it will generate with default extension: html.
pub fn generate<T: Serialize>(
    hb: &Handlebars,
    template_name: &str,
    data: &T,
    folder_path: &str,
    file_name: &str,
) -> Result<(), Error> {
    let rendered = hb.render(template_name, data)?;
    let file_name_with_extension = get_filename_with_ext(file_name, "html");
    println!("generating: {}", file_name_with_extension);
    generate_rendered_template(&rendered, folder_path, &file_name_with_extension)
}