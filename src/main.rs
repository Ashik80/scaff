use std::{env, fs};

use axum::{extract::Path, response::Html, routing::get, Router};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use crate::{component_creator::ComponentCreator, initializer::Initializer};

mod component_creator;
mod content_generators;
mod dir_file_generators;
mod initializer;
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
            Initializer::new(&args[2]).init();
        }
        "page" => {
            ComponentCreator::new(&args[2]).create_page_files();
        }
        "component" => {
            ComponentCreator::new(&args[2]).create_component_file();
        }
        "run" => {
            start_server();
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

#[tokio::main]
async fn start_server() {
    let router = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/styles", ServeDir::new("styles"))
        .nest_service("/src", ServeDir::new("src"))
        .route("/", get(index_page))
        .route("/api/page/*path", get(handle_page_route))
        .fallback(index_page);
    let addr = format!(
        "127.0.0.1:{}",
        env::var("PORT").unwrap_or("3000".to_string())
    );
    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind server");
    println!(
        "listening on port: {}",
        listener.local_addr().expect("failed to print port")
    );
    axum::serve(listener, router)
        .await
        .expect("failed to start server");
}

async fn index_page() -> Html<String> {
    let html = fs::read_to_string("index.html").expect("failed to read file");
    Html(html)
}

async fn handle_page_route(Path(mut path): Path<String>) -> Html<String> {
    let page_path = format!("src/page/{}/{}.html", path, path);
    let page_html = fs::read_to_string(page_path).expect("failed to read file");
    if path == "home" {
        path = "home-page".to_string();
    }
    let html = format!("<{}>{}</{}>", path, page_html, path);
    Html(html)
}
