#![windows_subsystem = "windows"]

mod gui;
use gui::State;

use iced::{Font, Size};

use std::fs;
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

const ICON_BYTES: &[u8] = include_bytes!("icon.ico");
const SERVER_LIST: &str = include_str!("serverlist.json");

fn read_serverlist() -> Vec<Server> {
    let data: ServerList =
        serde_json::from_str(&SERVER_LIST).expect("Failed to parse serverlist.json");

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
    .window(iced::window::Settings {
        icon: Some(iced::window::icon::from_file_data(ICON_BYTES, None).expect("Invalid icon")),
        ..Default::default()
    })
    .default_font(Font {
        family: iced::font::Family::Monospace,
        ..Font::DEFAULT
    })
    .resizable(false)
    .run()
}
