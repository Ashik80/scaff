use crate::component_creator::ComponentCreator;

mod component_creator;
mod string_case;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let tool_name = &args[0];

    if args.len() < 2 {
        display_usage(tool_name);
        std::process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "help" => {
            display_usage(tool_name);
        }
        "init" => {
            // TODO
            println!("initialize project directory")
        }
        "page" | "component" => {
            ComponentCreator::new(command, &args[2]).create_component_file();
        }
        _ => {
            display_usage(tool_name);
            std::process::exit(1);
        }
    }
}

fn display_usage(tool_name: &str) {
    println!(
        r#"Usage:
    {} component <component_name>
    {} page <page_name>"#,
        tool_name, tool_name
    );
}
