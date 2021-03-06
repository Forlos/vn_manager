use iced::{button, container, text_input, Background, Color, Vector};

const DARK: Color = Color::from_rgb(
    0x19 as f32 / 255.0,
    0x1B as f32 / 255.0,
    0x28 as f32 / 255.0,
);

const DARK_FOCUSED: Color = Color::from_rgb(
    0x29 as f32 / 255.0,
    0x2B as f32 / 255.0,
    0x38 as f32 / 255.0,
);

const DARK_BUTTON_FOCUSED: Color = Color::from_rgb(
    0x2C as f32 / 255.0,
    0x2F as f32 / 255.0,
    0x3B as f32 / 255.0,
);

const DARK_SELECTION: Color = Color::from_rgb(
    0x82 as f32 / 255.0,
    0xAA as f32 / 255.0,
    0xFF as f32 / 255.0,
);

const TEXT_COLOR: Color = Color::from_rgb(
    0x82 as f32 / 255.0,
    0x8B as f32 / 255.0,
    0xB8 as f32 / 255.0,
);

const BORDER_COLOR: Color = Color::from_rgb(
    0x13 as f32 / 255.0,
    0x14 as f32 / 255.0,
    0x21 as f32 / 255.0,
);

pub(crate) struct Dark;
impl container::StyleSheet for Dark {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(DARK)),
            border_width: 1,
            border_color: BORDER_COLOR,
            text_color: Some(TEXT_COLOR),
            border_radius: 0,
        }
    }
}
impl button::StyleSheet for Dark {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Vector::new(0.0, 0.0),
            background: Some(Background::Color(DARK_FOCUSED)),
            border_radius: 0,
            border_width: 1,
            border_color: BORDER_COLOR,
            text_color: TEXT_COLOR,
        }
    }
    fn hovered(&self) -> button::Style {
        let active = self.active();

        button::Style {
            shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
            background: Some(Background::Color(DARK_BUTTON_FOCUSED)),
            ..active
        }
    }
    fn pressed(&self) -> button::Style {
        button::Style {
            shadow_offset: Vector::default(),
            ..self.active()
        }
    }
    fn disabled(&self) -> button::Style {
        let active = self.active();

        button::Style {
            shadow_offset: Vector::default(),
            background: active.background.map(|background| match background {
                Background::Color(color) => Background::Color(Color {
                    a: color.a * 0.1,
                    ..color
                }),
            }),
            text_color: Color {
                a: active.text_color.a * 0.1,
                ..active.text_color
            },
            ..active
        }
    }
}

impl text_input::StyleSheet for Dark {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(HEADER),
            border_radius: 0,
            border_width: 1,
            border_color: BORDER_COLOR,
        }
    }
    fn focused(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(DARK_FOCUSED),
            border_radius: 0,
            border_width: 1,
            border_color: BORDER_COLOR,
        }
    }
    fn placeholder_color(&self) -> Color {
        Color {
            a: 0.1,
            ..TEXT_COLOR
        }
    }
    fn value_color(&self) -> Color {
        TEXT_COLOR
    }
    fn selection_color(&self) -> Color {
        DARK_SELECTION
    }
    fn hovered(&self) -> text_input::Style {
        self.focused()
    }
}

const HEADER: Color = Color::from_rgb(
    0x1B as f32 / 255.0,
    0x1D as f32 / 255.0,
    0x2C as f32 / 255.0,
);

const HEADER_TEXT_HOVER: Color = Color::from_rgb(
    0x96 as f32 / 255.0,
    0x9F as f32 / 255.0,
    0xCB as f32 / 255.0,
);

const HEADER_TEXT_PRESSED: Color = Color::from_rgb(
    0x96 as f32 / 255.0,
    0x9F as f32 / 255.0,
    0xCB as f32 / 255.0,
);

pub(crate) struct Header;
impl container::StyleSheet for Header {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(HEADER)),
            border_width: 1,
            border_color: BORDER_COLOR,
            text_color: Some(Color { ..TEXT_COLOR }),
            border_radius: 0,
        }
    }
}
impl button::StyleSheet for Header {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Vector::new(0.0, 0.0),
            background: Some(Background::Color(HEADER)),
            border_radius: 0,
            border_width: 0,
            border_color: Color::TRANSPARENT,
            text_color: TEXT_COLOR,
        }
    }
    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: HEADER_TEXT_HOVER,
            ..self.active()
        }
    }
    fn pressed(&self) -> button::Style {
        button::Style {
            text_color: HEADER_TEXT_PRESSED,
            ..self.active()
        }
    }
    fn disabled(&self) -> button::Style {
        let active = self.active();

        button::Style {
            shadow_offset: Vector::default(),
            background: active.background.map(|background| match background {
                Background::Color(color) => Background::Color(Color {
                    a: color.a * 0.1,
                    ..color
                }),
            }),
            text_color: Color {
                a: active.text_color.a * 0.1,
                ..active.text_color
            },
            ..active
        }
    }
}

const ERROR_TEXT_COLOR: Color = Color::from_rgb(
    0x80 as f32 / 255.0,
    0x20 as f32 / 255.0,
    0x20 as f32 / 255.0,
);

pub(crate) struct Error;
impl container::StyleSheet for Error {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(DARK)),
            border_width: 0,
            border_color: Color::TRANSPARENT,
            text_color: Some(ERROR_TEXT_COLOR),
            border_radius: 0,
        }
    }
}

const SUCCESS_TEXT_COLOR: Color = Color::from_rgb(
    0x20 as f32 / 255.0,
    0x80 as f32 / 255.0,
    0x20 as f32 / 255.0,
);

pub(crate) struct Success;
impl container::StyleSheet for Success {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(DARK)),
            border_width: 0,
            border_color: Color::TRANSPARENT,
            text_color: Some(SUCCESS_TEXT_COLOR),
            border_radius: 0,
        }
    }
}

const LIST_TEXT_HOVER: Color = Color::from_rgb(
    0xA9 as f32 / 255.0,
    0xB2 as f32 / 255.0,
    0xDF as f32 / 255.0,
);

const LIST_TEXT_PRESSED: Color = Color::from_rgb(
    0xA9 as f32 / 255.0,
    0xB2 as f32 / 255.0,
    0xDF as f32 / 255.0,
);

pub(crate) struct List;
impl button::StyleSheet for List {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Vector::new(0.0, 0.0),
            background: Some(Background::Color(DARK)),
            border_radius: 0,
            border_width: 0,
            border_color: Color::BLACK,
            text_color: TEXT_COLOR,
        }
    }
    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: LIST_TEXT_HOVER,
            ..self.active()
        }
    }
    fn pressed(&self) -> button::Style {
        button::Style {
            text_color: LIST_TEXT_PRESSED,
            ..self.active()
        }
    }
    fn disabled(&self) -> button::Style {
        let active = self.active();

        button::Style {
            shadow_offset: Vector::default(),
            background: active.background.map(|background| match background {
                Background::Color(color) => Background::Color(Color {
                    a: color.a * 0.1,
                    ..color
                }),
            }),
            text_color: Color {
                a: active.text_color.a * 0.1,
                ..active.text_color
            },
            ..active
        }
    }
}
