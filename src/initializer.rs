use core::panic;
use std::{
    fs::{self},
    path::Path,
};

use crate::{
    content_generators::{
        generate_favicon_selector_content, generate_global_css_content, generate_home_css_content,
        generate_home_html_content, generate_html_content, generate_page_component_js_content,
        generate_page_element_class_content, generate_router_js_content,
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

        // Generate router.js
        let router_js_file_name = "router.js";
        let router_js_content = generate_router_js_content().as_bytes();
        let route_js_path =
            match scaffold_dir_file(&services_dir, router_js_file_name, router_js_content) {
                Ok(path) => path,
                Err(e) => panic!("{}", e),
            };

        // Generate shadow-dom.js
        let shadow_dom_js_file = "shadow-dom.js";
        let shadow_dom_js_content = generate_page_element_class_content().as_bytes();
        let shadow_dom_path =
            match scaffold_dir_file(&services_dir, shadow_dom_js_file, shadow_dom_js_content) {
                Ok(path) => path,
                Err(e) => panic!("{}", e),
            };

        // Generate home page files
        let home_dir = src_dir.join("page").join("home");
        let home_html_file_name = "home.html";
        let home_html_content = generate_home_html_content().as_bytes();
        let home_html_path =
            match scaffold_dir_file(&home_dir, home_html_file_name, home_html_content) {
                Ok(path) => path,
                Err(e) => panic!("{}", e),
            };
        let home_js_file_name = "home.js";
        let home_js_content =
            generate_page_component_js_content("home-page", "homePage", "HomePage");
        let home_js_path =
            match scaffold_dir_file(&home_dir, home_js_file_name, home_js_content.as_bytes()) {
                Ok(path) => path,
                Err(e) => panic!("{}", e),
            };
        let home_css_file_name = "home.css";
        let home_css_content = generate_home_css_content().as_bytes();
        let home_css_path = match scaffold_dir_file(&home_dir, home_css_file_name, home_css_content)
        {
            Ok(path) => path,
            Err(e) => panic!("{}", e),
        };

        // Display generated files
        println!(
            r#"Generated files:
    {}
    {}
    {}
    {}
    {}
    {}
    {}
    {}
    {}"#,
            html_filepath.display(),
            assets_dir.display(),
            styles_file_path.display(),
            favicon_selector_path.display(),
            shadow_dom_path.display(),
            route_js_path.display(),
            home_html_path.display(),
            home_js_path.display(),
            home_css_path.display()
        )
    }
}
