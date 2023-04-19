pub mod views;
pub mod partials;

use std::{env, path::PathBuf, path::Path};
use relative_path::RelativePath;

// Provide full file path for Handlebars templates
// Ensures any changes to directory structure can be handled by simply changing one string
pub fn get_template_path(path: &'static str) -> PathBuf {
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push(Path::new("./templates/"));
    RelativePath::new(path).to_path(current_dir)
}