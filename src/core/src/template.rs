use std::path::PathBuf;

// The configuration of how a template is been stored and processed.
pub struct Profile {
    label: String,
    path: PathBuf,
    raw: Template,
}

impl Profile {}

// Text template for generating the substitude content.
struct Template {
    raw: String,
    replace: Option<HashMap<String, String>>,
}

impl Template {
    fn new(raw: String) -> Self {
        Self { raw, replace: None }
    }
}
