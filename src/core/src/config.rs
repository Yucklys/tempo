use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::template::Profile;
use std::collections::HashMap;
use std::fs;
use std::io;
use crate::Opts;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub profile_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub enum LoadError {
    FileError,
    FormatError,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            profile_dir: Config::path().join("user"),
        }
    }
}

impl Config {
    fn path() -> PathBuf {
        dirs::data_dir().unwrap().join("tempo")
    }

    /// Load configuration.
    pub async fn load() -> Result<Config, LoadError> {
        use async_std::prelude::*;

        let mut contents = String::new();
        let mut file = async_std::fs::File::open(Self::path().join("config.toml")).await.map_err(|_| LoadError::FileError)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::FileError)?;

        toml::from_str(&contents).map_err(|_| LoadError::FormatError)
    }

    /// Load configuration with terminal input.
    pub async fn load_extend(opts: Opts) -> Result<(Config, Opts), LoadError> {
        Config::load().await.map(|c| (c, opts))
    }

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
