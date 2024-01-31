mod state_file;
use state_file::State;

use iced::application;
use iced::executor;
use iced::theme::{self, Theme};
use iced::widget::canvas;
use iced::window;
use iced::{
    Application, Color, Command, Element, Length, Settings, Subscription,
};

use std::time::Instant;

pub fn main() -> iced::Result {
    env_logger::builder().format_timestamp(None).init();

    SolarSystem::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct SolarSystem {
    state: State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(Instant),
}

impl Application for SolarSystem {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            SolarSystem {
                state: State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Meow Solar")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(instant) => {
                self.state.update(instant);
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        canvas(&self.state)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn style(&self) -> theme::Application {
        fn dark_background(_theme: &Theme) -> application::Appearance {
            application::Appearance {
                background_color: Color::BLACK,
                text_color: Color::WHITE,
            }
        }

        theme::Application::from(dark_background as fn(&Theme) -> _)
    }

    fn subscription(&self) -> Subscription<Message> {
        window::frames().map(Message::Tick)
    }
}

