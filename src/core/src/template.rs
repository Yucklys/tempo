mod date_time;
mod raw;

use crate::template::date_time::DateTime;
use crate::template::raw::Raw;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::path::PathBuf;

// The configuration of how a template is been stored and processed.
#[derive(Deserialize, Serialize, Default, Clone, Debug, Eq, PartialEq)]
pub struct Profile {
    pub label: String,
    path: PathBuf,
    matches: Vec<Template>,
}

impl std::fmt::Display for Profile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl Profile {
    pub fn new(label: &str, path: PathBuf, matches: Vec<Template>) -> Self {
        Profile {
            label: label.to_string(),
            path,
            matches,
        }
    }

    pub fn add_match(mut self, template: Template) -> Self {
        self.matches.push(template);
        self
    }

    pub fn apply(&self, input: &str) -> String {
        let mut formatted = input.to_string();
        for t in &self.matches {
            if t.is_enabled() {
                formatted = t.format(formatted);
            }
        }
        formatted
    }

    pub fn get_templates(&self) -> &Vec<Template> {
        &self.matches
    }

    pub fn get_templates_mut(&mut self) -> &mut Vec<Template> {
        &mut self.matches
    }
}

// Text template.
#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum Template {
    Raw(Raw),
    DateTime(DateTime),
}

impl Template {
    pub fn raw(raw: &str, replace: &str) -> Self {
        Template::Raw(Raw {
            raw: raw.to_string(),
            replace: replace.to_string(),
            ..Raw::default()
        })
    }
    pub fn date_time(raw: &str, format: &str) -> Self {
        Template::DateTime(DateTime {
            raw: raw.to_string(),
            format: format.to_string(),
            ..DateTime::default()
        })
    }

    pub fn get_value(&self) -> &dyn Format {
        match self {
            Template::Raw(t) => t,
            Template::DateTime(t) => t,
        }
    }

    pub fn get_value_mut(&mut self) -> &mut dyn Format {
        match self {
            Template::Raw(t) => t,
            Template::DateTime(t) => t,
        }
    }

    pub fn format(&self, input: String) -> String {
        self.get_value().format(input)
    }

    pub fn is_enabled(&self) -> bool {
        self.get_value().is_enabled()
    }

    pub fn set_enabled(&mut self, is_enabled: bool) {
        self.get_value_mut().set_enabled(is_enabled)
    }
}

impl std::fmt::Display for Template {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_value().to_string())
    }
}

impl Default for Template {
    fn default() -> Self {
        Template::Raw(Raw::default())
    }
}

pub trait Format: std::fmt::Display {
    fn format(&self, input: String) -> String;
    fn is_enabled(&self) -> bool;
    fn set_enabled(&mut self, is_enabled: bool);
}

#[derive(Debug, Clone)]
pub enum TemplateError {
    FormatError,
    MatchError,
}

fn default_true() -> bool {
    true
}
