use core::panic;
use std::path::Path;

use crate::{
    content_generators::{generate_component_js_content, generate_page_component_js_content},
    dir_file_generators::scaffold_dir_file,
    string_case::StringConverter,
};

pub struct ComponentCreator {
    class_name: String,
    template_name: String,
    component_name: String,
}

impl ComponentCreator {
    pub fn new(component_name: &str) -> Self {
        Self {
            class_name: component_name.to_pascal_case(),
            template_name: component_name.to_camel_case(),
            component_name: component_name.to_string(),
        }
    }

    pub fn create_component_file(&self) {
        let component_dir = Path::new("src")
            .join("component")
            .join(&self.component_name);
        let js_file_name = format!("{}.js", self.component_name);
        let js_content = generate_component_js_content(
            &self.component_name,
            &self.template_name,
            &self.class_name,
        );
        let js_file_path =
            match scaffold_dir_file(&component_dir, &js_file_name, js_content.as_bytes()) {
                Ok(path) => path,
                Err(e) => panic!("{}", e),
            };

        // Display files generated
        println!(
            r#"Generated component file:
    {}"#,
            js_file_path.display()
        )
    }

    pub fn create_page_files(&self) {
        let page_dir = Path::new("src").join("page").join(&self.component_name);
        let html_file_name = format!("{}.html", self.component_name);
        let html_content = "".as_bytes();
        let html_path = match scaffold_dir_file(&page_dir, &html_file_name, html_content) {
            Ok(path) => path,
            Err(e) => panic!("{}", e),
        };

        let js_file_name = format!("{}.js", self.component_name);
        let js_content = generate_page_component_js_content(
            &self.component_name,
            &self.template_name,
            &self.class_name,
        );
        let js_path = match scaffold_dir_file(&page_dir, &js_file_name, js_content.as_bytes()) {
            Ok(path) => path,
            Err(e) => panic!("{}", e),
        };

        let css_file_name = format!("{}.css", self.component_name);
        let css_content = "".as_bytes();
        let css_path = match scaffold_dir_file(&page_dir, &css_file_name, css_content) {
            Ok(path) => path,
            Err(e) => panic!("{}", e),
        };

        println!(
            r#"Generated page files:
    {}
    {}
    {}"#,
            html_path.display(),
            js_path.display(),
            css_path.display()
        )
    }
}
