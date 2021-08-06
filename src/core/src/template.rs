mod raw;
mod date_time;

use std::path::PathBuf;
use std::collections::HashMap;
use crate::template::raw::Raw;
use crate::template::date_time::DateTime;
use serde::{Serialize, Deserialize};
use std::fmt::Formatter;

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
            label: label.to_string(), path, matches
        }
    }

    pub fn add_match(mut self, template: Template) -> Self {
        self.matches.push(template);
        self
    }

    pub fn apply(&self, input: &str) -> String {
        let mut formatted = input.to_string();
        for t in &self.matches {
            formatted = t.format(formatted);
        }
        formatted
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
            raw: raw.to_string(), replace: replace.to_string()
        })
    }
    pub fn date_time(raw: &str, format: &str) -> Self {
        Template::DateTime(DateTime {
            raw: raw.to_string(),
            format: format.to_string(),
        })
    }
}

impl Format for Template {
    fn format(&self, input: String) -> String {
        match self {
            Template::Raw(t) => t.format(input),
            Template::DateTime(t) => t.format(input),
        }
    }
}

trait Format {
    fn format(&self, input: String) -> String;
}

#[derive(Debug, Clone)]
pub enum TemplateError {
    FormatError,
    MatchError
}
