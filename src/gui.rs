use iced::{
    self,
    Length::Fill,
    Padding, Task,
    widget::{
        Space, checkbox, column, container, row,
        scrollable::{Direction, Scrollbar},
        text,
    },
};

use crate::{
    Server, revert,
    style::{
        ACCENT, BG_RAISED, BORDER_SUBTLE, TEXT_MUTED, TEXT_PRIMARY, app_background, ghost_button,
        modern_checkbox, primary_button, slim_scrollable,
    },
    write_to_host_file,
};

#[derive(Default)]
pub struct State {
    pub servers: Vec<Server>,
    pub current_server: Option<Server>,
}

#[derive(Debug, Clone)]
pub enum Message {
    UpdateServerSelection(Server),
    WriteToHostFile,
    Revert,
}

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::UpdateServerSelection(server) => {
            state.current_server = Some(server);
            Task::none()
        }
        Message::WriteToHostFile => {
            write_to_host_file(
                state.servers.as_ref(),
                state.current_server.as_ref().unwrap(),
            );
            Task::none()
        }
        Message::Revert => {
            revert();
            state.current_server = None;
            Task::none()
        }
    }
}

pub fn view(state: &State) -> iced::Element<'_, Message> {
    let header = column![
        text("SOUL GUARD").size(22).color(TEXT_PRIMARY),
        text("Select a region to route traffic")
            .size(12)
            .color(TEXT_MUTED),
    ]
    .spacing(4);

    let mut server_list = column![].spacing(6);

    for entry in state.servers.iter().rev().skip(6) {
        let is_selected = state
            .current_server
            .as_ref()
            .map_or(false, |s| s.url == entry.url);

        let row_content = row![
            checkbox(
                state
                    .current_server
                    .as_ref()
                    .map_or(false, |s| s.url == entry.url)
            )
            .style(modern_checkbox)
            .on_toggle(|_| Message::UpdateServerSelection(entry.clone())),
            column![
                text(&entry.pretty_name).size(14).color(TEXT_PRIMARY),
                text(&entry.url).size(11).color(TEXT_MUTED),
            ]
            .spacing(2),
            Space::new().width(Fill),
            if is_selected {
                text("ACTIVE").size(10).color(ACCENT)
            } else {
                text("").size(10).color(TEXT_MUTED)
            },
        ]
        .spacing(12)
        .align_y(iced::alignment::Vertical::Center);

        let card = container(row_content)
            .padding(Padding::from([10, 14]))
            .style(move |_theme: &iced::Theme| iced::widget::container::Style {
                background: Some(iced::Background::Color(if is_selected {
                    iced::Color { a: 0.15, ..ACCENT }
                } else {
                    BG_RAISED
                })),
                border: iced::Border {
                    color: if is_selected { ACCENT } else { BORDER_SUBTLE },
                    width: 1.0,
                    radius: 5.0.into(),
                },
                ..Default::default()
            });

        server_list = server_list.push(card);
    }

    let scrollable = iced::widget::scrollable(server_list)
        .anchor_left()
        .height(Fill)
        .width(Fill)
        .style(slim_scrollable)
        .direction(Direction::Vertical(
            Scrollbar::new().width(4).scroller_width(4),
        ));

    let revert_button = iced::widget::button("Revert")
        .width(130)
        .style(ghost_button)
        .on_press(Message::Revert);

    let apply_button = iced::widget::button("Apply Changes")
        .width(150)
        .on_press(Message::WriteToHostFile)
        .style(primary_button);

    let footer = row![revert_button, Space::new().width(Fill), apply_button,]
        .align_y(iced::alignment::Vertical::Center);

    let content = column![
        header,
        Space::new().height(16),
        scrollable,
        Space::new().height(16),
        footer,
    ]
    .padding(Padding::from([24, 20]));

    container(content)
        .style(app_background)
        .width(Fill)
        .height(Fill)
        .into()
}
