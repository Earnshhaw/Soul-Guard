#![windows_subsystem = "windows"]

mod gui;
use gui::State;

use iced::{Font, Size, window};

use std::{fs, io};
mod style;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub url: String,
    pub pretty_name: String,
    pub ping_check: String,
}

#[derive(Debug, Deserialize)]
struct ServerList {
    regions: Vec<ServerRaw>,
}

#[derive(Debug, Deserialize)]
struct ServerRaw {
    name: String,
    endpoint: String,
    ping_check: String,
}

fn read_serverlist() -> Vec<Server> {
    let contents =
        std::fs::read_to_string("serverlist.json").expect("Failed to read serverlist.json");

    let data: ServerList =
        serde_json::from_str(&contents).expect("Failed to parse serverlist.json");

    data.regions
        .into_iter()
        .map(|region| Server {
            url: format!("gamelift.{}.com", region.endpoint),
            pretty_name: region.name,
            ping_check: region.ping_check,
        })
        .collect()
}

pub fn revert() {
    let path = r"C:\Windows\System32\drivers\etc\hosts";
    let contents = fs::read_to_string(path).expect("Failed to read contents of the host file");
    let mut lines = contents
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
    lines.truncate(21);
    fs::write(path, lines.join("\r\n")).expect("Failed to write to host file");
}

pub fn write_to_host_file(servers: &Vec<Server>, selected_server: &Server) {
    let path = r"C:\Windows\System32\drivers\etc\hosts";

    let contents = fs::read_to_string(path).expect("Failed to read contents of the host file");

    let mut lines: Vec<String> = contents.lines().map(|l| l.to_string()).collect();

    lines.truncate(21);

    lines.push(String::new());
    lines.push(format!("# {}", selected_server.url));
    lines.push(format!("# {}", selected_server.ping_check));
    lines.push(String::new());

    for entry in servers.iter().filter(|x| x.url != selected_server.url) {
        lines.push(format!("0.0.0.0   {}", entry.url));
        lines.push(format!("0.0.0.0   {}", entry.ping_check));
        lines.push(String::new());
    }

    fs::write(path, lines.join("\r\n")).expect("Failed to write to host file");
}

fn main() -> iced::Result {
    env_logger::init();
    let icon = window::icon::from_file_data(include_bytes!("../icon.png"), None);

    iced::application(
        || State {
            servers: read_serverlist(),
            current_server: None,
        },
        gui::update,
        gui::view,
    )
    .title("Soul Guard")
    .window_size(Size::new(388.00, 580.00))
    .decorations(true)
    .resizable(false)
    .window(window::Settings {
        icon: Some(icon.unwrap()),
        ..Default::default()
    })
    .default_font(Font {
        family: iced::font::Family::Monospace,
        ..Font::DEFAULT
    })
    .run()
}
