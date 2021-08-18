mod config;
mod template;

pub use crate::config::*;
use clap::{AppSettings, Clap};
use std::collections::HashMap;
pub use template::{Profile, Template, TemplateError};

#[derive(Clap, Debug, Clone)]
#[clap(version = "0.1", author = "Yucklys <yucklys687@outlook.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    pub input: Option<String>,
    #[clap(long, short)]
    pub prefer: Option<String>,
    #[clap(long, short)]
    gui: bool,
}

pub async fn run_cli() -> Option<Opts> {
    let opts: Opts = Opts::parse();
    let input = opts.input.clone().unwrap_or(String::new());
    let config = Config::load().await.unwrap_or(Config::default());
    let profiles = config.get_profiles().unwrap();
    let prefer = &profiles
        .get(&opts.clone().prefer.unwrap_or(String::new()))
        .map(|p| p.clone());
    let output = apply_format(input.as_str(), prefer);

    if opts.gui {
        Some(opts)
    } else {
        match output {
            Ok(value) => println!("{}", value),
            Err(e) => match e {
                TemplateError::MatchError => {
                    println!("Cannot find preferred profile, maybe the name is not correct?")
                }
                TemplateError::FormatError => println!("Cannot format the input correctly."),
            },
        }
        None
    }
}

pub fn apply_format(input: &str, prefer: &Option<Profile>) -> Result<String, TemplateError> {
    if let Some(value) = &prefer {
        Ok(value.apply(input))
    } else {
        Err(TemplateError::MatchError)
    }
}

#[cfg(test)]
mod tests {
    use crate::template::*;
    use std::path::PathBuf;

    #[test]
    fn raw_template() {
        let mut profile = test_profile()
            .add_match(Template::raw("h", "hello"))
            .add_match(Template::raw("tem", "tempo"))
            .add_match(Template::date_time(":now", "%Y-%m-%d %H:%M:%S"));
        println!("{}", profile.apply("now is :now"));
        assert_eq!(profile.apply("h, tem"), "hello, tempo");
    }

    fn test_profile() -> Profile {
        Profile::new("test", PathBuf::from("test.txt"), vec![])
    }
}
