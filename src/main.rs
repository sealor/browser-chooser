use iced::{
    executor,
    subscription::events_with,
    theme,
    widget::{Button, Column, Container},
    window, Application, Color, Command, Element, Event, Length, Settings, Subscription, Theme,
};
use std::{env, process};

fn main() -> Result<(), iced::Error> {
    let settings = Settings {
        window: window::Settings {
            size: (100, 60),
            decorations: false,
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
    initially_focused: bool,
}

#[derive(Debug, Clone)]
enum Action {
    Open(String),
    HandleWindowEvent(window::Event),
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
                initially_focused: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Browser Chooser".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Action::Open(cmd) => {
                process::Command::new(cmd).args(env::args()).spawn().ok();
                window::close()
            }
            Action::HandleWindowEvent(window::Event::Focused) => {
                self.initially_focused = true;
                Command::none()
            }
            Action::HandleWindowEvent(window::Event::Unfocused) => {
                if self.initially_focused {
                    window::close()
                } else {
                    Command::none()
                }
            }
            Action::Cancel => window::close(),
            _ => Command::none(),
        }
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

    fn subscription(&self) -> Subscription<Self::Message> {
        events_with(|event, _| match event {
            Event::Window(window_event) => Some(Action::HandleWindowEvent(window_event)),
            _ => None,
        })
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
