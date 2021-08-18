mod recipe_tree;

use crate::gui::recipe_tree::RecipeTree;
use iced::{
    button, pick_list, text_input, Application, Button, Clipboard, Column, Command, Container,
    Element, HorizontalAlignment, Length, PickList, Row, Settings, Text, TextInput,
};
use std::collections::HashMap;
use tempo_core::{apply_format, Config, LoadError, Opts, Profile};

pub fn run_gui(flags: Opts) -> iced::Result {
    App::run(Settings {
        default_font: Some(include_bytes!("../../fonts/sarasa-regular.ttc")),
        ..Settings::with_flags(flags)
    })
}

#[derive(Debug)]
enum App {
    Loading,
    Loaded(State),
}

#[derive(Debug, Default)]
struct State {
    preferred_profile: Option<Profile>,
    profile_list: pick_list::State<Profile>,
    profiles: Vec<Profile>,
    output: String,
    input: text_input::State,
    input_value: String,
    saved_inputs: Vec<(String, button::State)>,
    recipe_tree: RecipeTree,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<(Config, Opts), LoadError>),
    InputChanged(String),
    InputSubmitted,
    GenerateOutput(String),
    ProfileSelected(Profile),
    TemplateToggle(usize, bool),
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = Opts;

    fn new(flags: Opts) -> (App, Command<Message>) {
        (
            App::Loading,
            Command::perform(Config::load_extend(flags), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        String::from("Tempo")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match self {
            App::Loading => {
                match message {
                    Message::Loaded(Ok((config, opts))) => {
                        let profiles = config.get_profiles().unwrap_or(HashMap::new());
                        let default_profile = profiles.keys().nth(0).unwrap();
                        let preferred_profile = profiles
                            .get(&opts.prefer.unwrap_or(default_profile.clone()))
                            .map(|p| p.clone());
                        *self = App::Loaded(State {
                            profiles: profiles
                                .values()
                                .map(|p| p.clone())
                                .collect::<Vec<Profile>>(),
                            preferred_profile,
                            input_value: opts.input.unwrap_or(String::new()),
                            recipe_tree: RecipeTree::new(),
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
                    Message::InputSubmitted => {
                        state
                            .saved_inputs
                            .push((state.input_value.clone(), button::State::new()));
                        state.input_value = String::new();
                    }
                    Message::GenerateOutput(input) => {
                        let output = apply_format(&input, &state.preferred_profile);
                        match output {
                            Ok(v) => state.output = v,
                            Err(_) => state.output = format!("Format failed! {:?}", state.profiles),
                        }
                    }
                    Message::ProfileSelected(profile) => state.preferred_profile = Some(profile),
                    Message::TemplateToggle(i, is_enabled) => {
                        let templates = match &mut state.preferred_profile {
                            Some(p) => p.get_templates_mut(),
                            None => panic!("No preferred profile"),
                        };
                        templates.get_mut(i).unwrap().set_enabled(is_enabled)
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
                saved_inputs,
                output,
                profile_list,
                profiles,
                preferred_profile,
                recipe_tree,
                ..
            }) => {
                let recipes = {
                    let templates = match preferred_profile {
                        Some(p) => p.get_templates_mut(),
                        None => panic!("No preferred profile"),
                    };

                    recipe_tree.view(templates)
                };
                let input_box = TextInput::new(input, "Input", input_value, Message::InputChanged)
                    .padding(15)
                    .size(20)
                    .on_submit(Message::InputSubmitted);
                let inputs = saved_inputs.iter_mut().fold(
                    Column::new().spacing(10),
                    |column: Column<Message>, (input, state)| {
                        column.push(
                            Button::new(state, Text::new(input.clone()))
                                .on_press(Message::GenerateOutput(input.clone())),
                        )
                    },
                );
                let output = Text::new(output.to_string()).size(20);
                let profile_picker = PickList::new(
                    profile_list,
                    profiles.to_owned(),
                    preferred_profile.clone(),
                    Message::ProfileSelected,
                )
                .width(Length::Fill);

                let content = Column::new()
                    .spacing(20)
                    .width(Length::FillPortion(3))
                    .push(input_box)
                    .push(inputs);
                let profile_view = Column::new()
                    .spacing(20)
                    .width(Length::FillPortion(7))
                    .push(profile_picker)
                    .push(recipes)
                    .push(output);
                let row = Row::new()
                    .padding(30)
                    .spacing(20)
                    .push(content)
                    .push(profile_view);

                row.into()
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
