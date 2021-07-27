use iced::{Application, Command, Clipboard, Element, Settings, Container, Text, HorizontalAlignment, Length, TextInput, text_input, Column};
use tempo_core::{Config, LoadError, run_cli, Opts};

pub fn run_gui(flags: Opts) -> iced::Result {
    App::run(Settings::with_flags(flags))
}

#[derive(Debug)]
enum App {
    Loading,
    Loaded(State)
}

#[derive(Debug, Default)]
struct State {
    input: text_input::State,
    input_value: String
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<(Config, Opts), LoadError>),
    InputChanged(String)
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = Opts;

    fn new(flags: Opts) -> (App, Command<Message>) {
        (App::Loading, Command::perform(Config::load_extend(flags), Message::Loaded))
    }

    fn title(&self) -> String {
        String::from("Tempo")
    }

    fn update(&mut self, message: Self::Message, clipboard: &mut Clipboard) -> Command<Self::Message> {
        match self {
            App::Loading => {
                match message {
                    Message::Loaded(Ok((_config, opts))) => {
                        *self = App::Loaded(State {
                            input_value: opts.input,
                            ..State::default()
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        *self = App::Loaded(State::default());
                    }
                    _ => {}
                }

                Command::none()
            }
           App::Loaded(state) => {
               match message {
                   Message::InputChanged(value) => {
                       state.input_value = value;
                   }
                   _ => {}
               }

               Command::none()
           }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        match self {
            App::Loading => loading_message(),
            App::Loaded(State {
                input,
                input_value,
                        }) => {
                let input = TextInput::new(
                    input,
                    "Input",
                    input_value,
                    Message::InputChanged
                ).padding(15).size(30);

                let content = Column::new().push(input);

                content.into()
            }
        }
    }
}

fn loading_message<'a>() -> Element<'a, Message> {
    Container::new(
        Text::new("Loading...")
            .horizontal_alignment(HorizontalAlignment::Center)
            .size(50),
    )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .into()
}