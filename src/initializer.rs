use core::panic;
use std::{
    fs::{self},
    path::Path,
};

use crate::{
    content_generators::{
        generate_favicon_selector_content, generate_global_css_content, generate_html_content,
    },
    dir_file_generators::scaffold_dir_file,
    string_case::StringConverter,
};

pub struct Initializer {
    project_name: String,
}

impl Initializer {
    pub fn new(project_name: &str) -> Self {
        Self {
            project_name: project_name.to_string(),
        }
    }

    pub fn init(&self) {
        let project_dir = Path::new(&self.project_name).to_owned();
        let src_dir = project_dir.join("src");

        // Generate index.html
        let html_filename = "index.html";
        let title = self.project_name.as_str().to_title();
        let html_content = generate_html_content(&title);
        let html_filepath =
            match scaffold_dir_file(&project_dir, html_filename, html_content.as_bytes()) {
                Ok(path) => path,
                Err(err) => panic!("{}", err),
            };

        // Generate assets directory
        let assets_dir = project_dir.join("assets");
        fs::create_dir(&assets_dir).expect("failed to create assets directory");

        // Generate global.css
        let styles_dir = project_dir.join("styles");
        let style_filename = "global.css";
        let css_content = generate_global_css_content().as_bytes();
        let styles_file_path = match scaffold_dir_file(&styles_dir, style_filename, css_content) {
            Ok(path) => path,
            Err(e) => panic!("{}", e),
        };

        // Generate favicon-selector.js
        let services_dir = src_dir.join("services");
        let favicon_selector_filename = "favicon-selector.js";
        let favicon_selector_content = generate_favicon_selector_content().as_bytes();
        let favicon_selector_path = match scaffold_dir_file(
            &services_dir,
            favicon_selector_filename,
            favicon_selector_content,
        ) {
            Ok(path) => path,
            Err(e) => panic!("{}", e),
        };

        // Display generated files
        println!(
            r#"Generated files:
    {}
    {}
    {}
    {}"#,
            html_filepath.display(),
            assets_dir.display(),
            styles_file_path.display(),
            favicon_selector_path.display()
        )
    }
}
