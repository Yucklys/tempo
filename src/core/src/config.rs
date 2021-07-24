use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::template::Profile;
use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub profile_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let profile_dir = dirs::data_dir().unwrap().join("tempo");
        Self {
            profile_dir,
        }
    }
}

impl Config {
    pub fn get_profiles(&self) -> io::Result<HashMap<String, Profile>> {
        self.get_profiles_helper(self.profile_dir.clone())
    }

    fn get_profiles_helper(&self, dir: PathBuf) -> io::Result<HashMap<String, Profile>> {
        let mut profiles = HashMap::new();
        for e in fs::read_dir(dir)? {
            let entry = e?;
            let path = entry.path();
            if path.is_dir() {
                profiles.extend(self.get_profiles_helper(path)?)
            } else {
                let cur_profile: Profile = toml::from_str(fs::read_to_string(path)?.as_str())?;
                profiles.insert(cur_profile.label.clone(), cur_profile);
            }
        }

        Ok(profiles)
    }
}
