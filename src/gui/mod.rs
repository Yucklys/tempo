mod recipe_tree;

use crate::gui::recipe_tree::RecipeTree;
use eframe::{egui, epi};
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
    Loading(Config, Opts),
    Loaded(State),
}

#[derive(Debug, Default)]
struct State {
    preferred_profile: Option<Profile>,
    profile_list: pick_list::State<Profile>,
    profiles: Vec<Profile>,
    output: String,
    input: String,
    saved_inputs: Vec<(String, button::State)>,
    recipe_tree: RecipeTree,
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Tempo"
    }

    #[cfg(feature = "persistence")]
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        storage: Option<&dyn epi::Storage>,
    ) {
        if let Some(storage) = storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        match self {
            App::Loading(config, opts) => {
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
                    input: opts.input.unwrap_or(String::new()),
                    recipe_tree: RecipeTree::new(),
                    ..State::default()
                });
            }
            App::Loaded(state) => {
                match self {
                    App::Loading(..) => {
                        egui::CentralPanel::default().show(ctx, |ui| loading_message(ui));
                    }
                    App::Loaded(State {
                        input,
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
                        egui::SidePanel::left("input_panel").show(ctx, |ui| {
                            ui.add(egui::TextEdit::singleline(input).hint_text("Input"));
                            saved_inputs.iter().for_each(|input| {
                                ui.label(input);
                            });
                        });
                        egui::SidePanel::right("output_panel").show(ctx, |ui| {
                            ui.label(output);
                        });
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
                    }
                }
                match message {
                    Message::InputChanged(value) => {
                        state.input = value;
                    }
                    Message::InputSubmitted => {
                        state
                            .saved_inputs
                            .push((state.input.clone(), button::State::new()));
                        state.input = String::new();
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
            }
        }
    }
}

fn loading_message(ui: &mut egui::Ui) {
    ui.heading("Loading...");
}
