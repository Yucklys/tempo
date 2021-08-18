use crate::gui::Message;
use iced::{Align, Checkbox, Column, Element, Length, Row, Text};
use tempo_core::Template;

#[derive(Debug)]
pub struct RecipeTree {}

impl Default for RecipeTree {
    fn default() -> Self {
        RecipeTree {}
    }
}

impl RecipeTree {
    pub fn new() -> Self {
        RecipeTree {
            ..RecipeTree::default()
        }
    }

    pub fn view(&mut self, templates: &mut Vec<Template>) -> Element<Message> {
        templates
            .iter_mut()
            .enumerate()
            .fold(Column::new().spacing(10), |column, (i, template)| {
                let controls: Row<Message> = Row::new()
                    .spacing(10)
                    .align_items(Align::End)
                    .push(Text::new("UP"))
                    .push(Text::new("DOWN"));
                let row = Row::new()
                    .push(
                        Checkbox::new(
                            template.is_enabled(),
                            template.to_string(),
                            move |is_enabled| Message::TemplateToggle(i, is_enabled),
                        )
                        .width(Length::Fill),
                    )
                    .push(controls);
                column.push(row)
            })
            .into()
    }
}
