use crate::template::Format;
use chrono::Local;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct DateTime {
    pub raw: String,
    pub format: String,
}

impl Format for DateTime {
    fn format(&self, input: String) -> String {
        let dt = Local::now();
        let time = dt.format(&self.format).to_string();
        input.replace(&self.raw, &time)
    }
}