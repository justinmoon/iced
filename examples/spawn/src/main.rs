use iced::{
    button, executor, Align, Application, Button, Column, Command, Container,
    Element, Length, Settings, Subscription, Text,
};

mod spawn;

pub fn main() {
    Example::run(Settings::default())
}

#[derive(Debug)]
enum Example {
    On { button: button::State },
    Off { button: button::State },
}

#[derive(Debug, Clone)]
pub enum Message {
    Spawn,
    DaemonUpdate(spawn::Update),
}

impl Application for Example {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Example, Command<Message>) {
        (
            Example::Off {
                button: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Download progress - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Spawn => match self {
                Example::Off { .. } => {
                    *self = Example::On {
                        button: button::State::new(),
                    }
                }
                _ => {}
            },
            Message::DaemonUpdate(message) => match message {
                spawn::Update::On { .. } => {
                    println!("Received Update::On message");
                    *self = Example::On {
                        button: button::State::new(),
                    };
                }
                spawn::Update::Off { .. } => {
                    println!("Received Update::Off message");
                    *self = Example::Off {
                        button: button::State::new(),
                    };
                }
            },
        };

        // FIXME: what's this for?
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        match self {
            Example::On { .. } => {
                //download::file("https://speed.hetzner.de/100MB.bin")
                //.map(Message::DownloadProgressed)
                spawn::bitcoind("regtest").map(Message::DaemonUpdate)
            }
            _ => Subscription::none(),
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = Column::new()
            .spacing(10)
            .padding(10)
            .align_items(Align::Center)
            .push(match self {
                Example::On { button } => {
                    Button::new(button, Text::new("Kill"))
                }
                Example::Off { button } => {
                    Button::new(button, Text::new("Start"))
                        .on_press(Message::Spawn)
                }
            });

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
