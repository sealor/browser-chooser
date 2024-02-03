use iced::{
    executor, theme,
    widget::{Button, Column, Container},
    window, Application, Color, Command, Element, Length, Settings, Theme,
};
use std::process::{self, exit};

fn main() -> Result<(), iced::Error> {
    let settings = Settings {
        window: window::Settings {
            size: (100, 60),
            ..window::Settings::default()
        },
        default_text_size: 12.0,
        ..Settings::default()
    };
    BrowserList::run(settings)
}

struct Browser {
    title: String,
    command: String,
}

struct BrowserList {
    list: Vec<Browser>,
}

#[derive(Debug, Clone)]
enum Action {
    Open(String),
    Cancel,
}

impl Application for BrowserList {
    type Message = Action;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (BrowserList, Command<Self::Message>) {
        (
            Self {
                list: vec![
                    Browser {
                        title: "Mozilla Firefox".to_string(),
                        command: "firefox".to_string(),
                    },
                    Browser {
                        title: "Google Chrome".to_string(),
                        command: "chrome".to_string(),
                    },
                ],
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Browser Chooser".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        if let Action::Open(cmd) = message {
            process::Command::new(cmd).spawn().ok();
        };
        exit(0);
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let mut column = Column::new();

        fn new_button(title: &str, action: Action) -> Button<'_, Action> {
            Button::new(title)
                .on_press(action)
                .padding(1)
                .width(Length::Fill)
                .height(Length::Fill)
        }

        for browser in self.list.iter() {
            let action = Action::Open(browser.command.clone());
            let button = new_button(browser.title.as_str(), action);
            column = column.push(button);
        }

        let button = new_button("Cancel", Action::Cancel);
        column = column.push(button);

        Container::new(column).into()
    }

    fn theme(&self) -> Theme {
        Theme::custom(theme::Palette {
            background: Color::from_rgb(1.0, 1.0, 1.0),
            text: Color::BLACK,
            primary: Color::from_rgb(0.95, 0.95, 0.95),
            success: Color::from_rgb(0.0, 1.0, 0.0),
            danger: Color::from_rgb(1.0, 0.0, 0.0),
        })
    }
}
