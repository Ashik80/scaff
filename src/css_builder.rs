use std::{fs, path::PathBuf};

use crate::{
    content_generators::generate_style_sheet_from_css, dir_file_generators::scaffold_dir_file,
};

pub struct CSSBuilder {}

impl CSSBuilder {
    pub fn build_css() {
        let css_dir = std::path::Path::new("styles");
        let dir_files = fs::read_dir(css_dir).expect("failed to read styles directory");

        for file in dir_files {
            let file_entry = file.expect("failed to get directory entry");
            let mut file_path = file_entry.path();

            if file_path.is_file() {
                let extension = file_path.extension().expect("failed to get extension");
                let str = extension.to_str().unwrap_or("");
                if str == "css" {
                    let css = fs::read_to_string(&file_path).expect("failed to read file");
                    file_path.set_extension("js");
                    let file_name = file_path.file_name().expect("failed to get new path");
                    let file_name = file_name.to_str().unwrap_or("");
                    let splitted: Vec<&str> = file_name.split('.').collect();
                    let template_name = splitted[0];
                    let js_css = generate_style_sheet_from_css(template_name, &css);
                    match scaffold_dir_file(&PathBuf::from(css_dir), file_name, js_css.as_bytes()) {
                        Ok(path) => println!("Built file: {}", path.display()),
                        Err(err) => panic!("{}", err),
                    };
                }
            }
        }
    }
}
