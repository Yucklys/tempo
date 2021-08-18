use crate::template::{default_true, Format};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Deserialize, Serialize, Debug, Clone, Default, Eq, PartialEq)]
pub struct Raw {
    pub raw: String,
    pub replace: String,
    #[serde(default = "default_true")]
    pub is_enabled: bool,
}

impl Format for Raw {
    fn format(&self, input: String) -> String {
        input.replace(&self.raw, &self.replace)
    }
    fn is_enabled(&self) -> bool {
        self.is_enabled
    }
    fn set_enabled(&mut self, is_enabled: bool) {
        self.is_enabled = is_enabled;
    }
}

impl std::fmt::Display for Raw {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.raw, self.replace)
    }
}
