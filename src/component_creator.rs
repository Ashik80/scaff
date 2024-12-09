use core::panic;
use std::path::Path;

use crate::{
    content_generators::generate_component_js_content, dir_file_generators::scaffold_dir_file,
    string_case::StringConverter,
};

pub struct ComponentCreator {
    command: String,
    class_name: String,
    template_name: String,
    component_name: String,
}

impl ComponentCreator {
    pub fn new(command: &str, component_name: &str) -> Self {
        Self {
            command: command.to_string(),
            class_name: component_name.to_pascal_case(),
            template_name: component_name.to_camel_case(),
            component_name: component_name.to_string(),
        }
    }

    pub fn create_component_file(&self) {
        let component_path = Path::new("src")
            .join(&self.command)
            .join(&self.component_name);
        let js_file_name = format!("{}.js", self.component_name);
        let js_content = generate_component_js_content(
            &self.component_name,
            &self.template_name,
            &self.class_name,
        );
        let js_file_path =
            match scaffold_dir_file(&component_path, &js_file_name, js_content.as_bytes()) {
                Ok(path) => path,
                Err(e) => panic!("{}", e),
            };

        // Display files generated
        println!(
            r#"Created {}:
    {}"#,
            self.command,
            js_file_path.display()
        )
    }
}
