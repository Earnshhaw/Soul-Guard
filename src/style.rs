// style.rs — Soul Guard UI theme
// Dark industrial aesthetic: deep charcoal, sharp cyan accent, tight geometry.

use iced::{
    Border, Color, Shadow, Vector,
    widget::{
        button, checkbox, container,
        scrollable::{self, AutoScroll, Rail, Scroller, Status as ScrollStatus},
    },
};

// ─── Palette ────────────────────────────────────────────────────────────────

pub const BG_BASE: Color = Color::from_rgb(0.055, 0.059, 0.067); // #0E0F11
pub const BG_SURFACE: Color = Color::from_rgb(0.086, 0.094, 0.106); // #16181B
pub const BG_RAISED: Color = Color::from_rgb(0.118, 0.129, 0.145); // #1E2125

pub const ACCENT: Color = Color::from_rgb(0.0, 0.839, 0.769); // #00D6C4  cyan-teal
pub const ACCENT_DIM: Color = Color::from_rgb(0.0, 0.502, 0.463); // #008076

pub const TEXT_PRIMARY: Color = Color::from_rgb(0.918, 0.929, 0.941); // #EAEDF0
pub const TEXT_MUTED: Color = Color::from_rgb(0.42, 0.455, 0.502); // #6B7480

pub const BORDER_SUBTLE: Color = Color::from_rgb(0.165, 0.18, 0.204); // #2A2E34
pub const BORDER_ACCENT: Color = ACCENT;

// ─── Container: app_background ──────────────────────────────────────────────

pub fn app_background(theme: &iced::Theme) -> container::Style {
    let _ = theme;
    container::Style {
        background: Some(iced::Background::Color(BG_BASE)),
        text_color: Some(TEXT_PRIMARY),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        shadow: Shadow::default(),
        snap: false,
    }
}

// ─── Button: primary_button ─────────────────────────────────────────────────

pub fn primary_button(theme: &iced::Theme, status: button::Status) -> button::Style {
    let _ = theme;
    match status {
        button::Status::Active => button::Style {
            background: Some(iced::Background::Color(ACCENT)),
            text_color: Color::from_rgb(0.04, 0.04, 0.04),
            border: Border {
                color: ACCENT,
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: Shadow {
                color: Color { a: 0.35, ..ACCENT },
                offset: Vector::new(0.0, 3.0),
                blur_radius: 12.0,
            },
            snap: false,
        },
        button::Status::Hovered => button::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.0, 0.949, 0.871))),
            text_color: Color::from_rgb(0.04, 0.04, 0.04),
            border: Border {
                color: Color::from_rgb(0.0, 0.949, 0.871),
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: Shadow {
                color: Color { a: 0.55, ..ACCENT },
                offset: Vector::new(0.0, 4.0),
                blur_radius: 18.0,
            },
            snap: false,
        },
        button::Status::Pressed => button::Style {
            background: Some(iced::Background::Color(ACCENT_DIM)),
            text_color: TEXT_PRIMARY,
            border: Border {
                color: ACCENT_DIM,
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        },
        button::Status::Disabled => button::Style {
            background: Some(iced::Background::Color(BG_RAISED)),
            text_color: TEXT_MUTED,
            border: Border {
                color: BORDER_SUBTLE,
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        },
    }
}

// ─── Button: ghost_button ───────────────────────────────────────────────────

pub fn ghost_button(theme: &iced::Theme, status: button::Status) -> button::Style {
    let _ = theme;
    match status {
        button::Status::Active => button::Style {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            text_color: TEXT_MUTED,
            border: Border {
                color: BORDER_SUBTLE,
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        },
        button::Status::Hovered => button::Style {
            background: Some(iced::Background::Color(BG_RAISED)),
            text_color: TEXT_PRIMARY,
            border: Border {
                color: Color { a: 0.5, ..ACCENT },
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        },
        button::Status::Pressed => button::Style {
            background: Some(iced::Background::Color(BG_SURFACE)),
            text_color: ACCENT,
            border: Border {
                color: ACCENT,
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        },
        button::Status::Disabled => button::Style {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            text_color: Color {
                a: 0.3,
                ..TEXT_MUTED
            },
            border: Border {
                color: Color {
                    a: 0.2,
                    ..BORDER_SUBTLE
                },
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        },
    }
}

// ─── Checkbox: modern_checkbox ──────────────────────────────────────────────

pub fn modern_checkbox(theme: &iced::Theme, status: checkbox::Status) -> checkbox::Style {
    let _ = theme;
    match status {
        checkbox::Status::Active { is_checked } => checkbox::Style {
            background: if is_checked {
                iced::Background::Color(ACCENT)
            } else {
                iced::Background::Color(BG_RAISED)
            },
            icon_color: Color::from_rgb(0.04, 0.04, 0.04),
            border: Border {
                color: if is_checked { ACCENT } else { BORDER_SUBTLE },
                width: 1.5,
                radius: 3.0.into(),
            },
            text_color: Some(TEXT_PRIMARY),
        },
        checkbox::Status::Hovered { is_checked } => checkbox::Style {
            background: if is_checked {
                iced::Background::Color(Color::from_rgb(0.0, 0.949, 0.871))
            } else {
                iced::Background::Color(BG_RAISED)
            },
            icon_color: Color::from_rgb(0.04, 0.04, 0.04),
            border: Border {
                color: ACCENT,
                width: 1.5,
                radius: 3.0.into(),
            },
            text_color: Some(TEXT_PRIMARY),
        },
        checkbox::Status::Disabled { is_checked } => checkbox::Style {
            background: if is_checked {
                iced::Background::Color(ACCENT_DIM)
            } else {
                iced::Background::Color(BG_SURFACE)
            },
            icon_color: TEXT_MUTED,
            border: Border {
                color: Color {
                    a: 0.3,
                    ..BORDER_SUBTLE
                },
                width: 1.5,
                radius: 3.0.into(),
            },
            text_color: Some(TEXT_MUTED),
        },
    }
}

// ─── Scrollable: slim_scrollable ────────────────────────────────────────────

fn make_rail(scroller_color: Color) -> Rail {
    Rail {
        background: Some(iced::Background::Color(BG_SURFACE)),
        border: Border {
            color: BORDER_SUBTLE,
            width: 0.0,
            radius: 2.0.into(),
        },
        scroller: Scroller {
            background: iced::Background::Color(scroller_color),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 2.0.into(),
            },
        },
    }
}

fn make_auto_scroll() -> AutoScroll {
    AutoScroll {
        background: iced::Background::Color(BG_RAISED),
        border: Border {
            color: BORDER_SUBTLE,
            width: 1.0,
            radius: 4.0.into(),
        },
        shadow: Shadow::default(),
        icon: TEXT_MUTED,
    }
}

pub fn slim_scrollable(theme: &iced::Theme, _status: ScrollStatus) -> scrollable::Style {
    let _ = theme;

    scrollable::Style {
        container: container::Style {
            background: Some(iced::Background::Color(BG_SURFACE)),
            border: Border {
                color: BORDER_SUBTLE,
                width: 1.0,
                radius: 6.0.into(),
            },
            text_color: None,
            shadow: Shadow::default(),
            snap: false,
        },
        vertical_rail: make_rail(BORDER_SUBTLE),
        horizontal_rail: make_rail(BORDER_SUBTLE),
        gap: None,
        auto_scroll: make_auto_scroll(),
    }
}
