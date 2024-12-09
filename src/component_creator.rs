use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::string_case::StringCase;

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
        let base_path = format!("src/{}", self.command);
        let component_path = Path::new(&base_path).join(&self.component_name);
        fs::create_dir_all(&component_path).expect("failed to create directory");

        let (file_path, mut file) = self
            .create_file_in_path(&component_path)
            .expect("failed to create file");

        self.write_component_content(&mut file)
            .expect("failed to write to file");

        println!("Created {} {:?}", self.command, file_path)
    }

    fn create_file_in_path(
        &self,
        component_path: &Path,
    ) -> Result<(PathBuf, File), std::io::Error> {
        let file_path = component_path.join(format!("{}.js", self.component_name));
        let file = fs::File::create(&file_path)?;
        Ok((file_path, file))
    }

    fn write_component_content(&self, file: &mut File) -> Result<(), std::io::Error> {
        let component_content = format!(
            r#"const {}Template = document.createElement("template");
{}Template.innerHTML = ``;

export class {} extends HTMLElement {{
  constructor() {{
    super();
  }}
}}

customElements.define("{}", {});
"#,
            self.template_name,
            self.template_name,
            self.class_name,
            self.component_name,
            self.class_name
        );
        println!("\n\n{}\n\n", component_content);

        file.write_all(component_content.as_bytes())
    }
}
