use crate::template::{default_true, Format};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Deserialize, Serialize, Debug, Clone, Default, Eq, PartialEq)]
pub struct DateTime {
    pub raw: String,
    pub format: String,
    #[serde(default = "default_true")]
    pub is_enabled: bool,
}

impl Format for DateTime {
    fn format(&self, input: String) -> String {
        let dt = Local::now();
        let time = dt.format(&self.format).to_string();
        input.replace(&self.raw, &time)
    }

    fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    fn set_enabled(&mut self, is_enabled: bool) {
        self.is_enabled = is_enabled;
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.raw, self.format)
    }
}
