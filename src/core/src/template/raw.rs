use crate::template::Format;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct Raw {
    pub raw: String,
    pub replace: String
}

impl Format for Raw {
    fn format(&self, input: String) -> String {
        input.replace(&self.raw, &self.replace)
    }
}