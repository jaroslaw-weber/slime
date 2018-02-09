use handlebars::Handlebars;
use std::error::Error;
use fs_extra::file::*;
use serde::ser::Serialize;

/// Write html file
pub fn generate_rendered_html(
    content: &str,
    folder_path: &str,
    file_name: &str,
) -> Result<(), Box<Error>> {
    let full_path = format!("{}/{}.html", folder_path, file_name);
    write_all(full_path, content)?;
    Ok(())
}

/// Generate html file
pub fn generate<T: Serialize>(
    hb: &Handlebars,
    template_name: &str,
    data: &T,
    folder_path: &str,
    file_name: &str,
) -> Result<(), Box<Error>> {
    println!("generating: {}", file_name);
    let rendered = hb.render(template_name, data)?;
    generate_rendered_html(&rendered, folder_path, file_name)
}
