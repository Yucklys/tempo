mod config;
mod template;

use clap::{AppSettings, Clap};
use crate::config::Config;

#[derive(Clap)]
#[clap(version = "0.1", author = "Yucklys <yucklys687@outlook.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    input: String,
    #[clap(long, short, default_value = " ")]
    prefer: String,
}

pub fn run_cli() {
    let opts: Opts = Opts::parse();
    let input = opts.input.as_str();
    let config = Config::default();
    let profiles = config.get_profiles().unwrap();
    let mut output = String::new();
    if opts.prefer != " ".to_string() {
        let preferred_profile = profiles.get(&opts.prefer);
        match preferred_profile {
            Some(p) => output = p.apply(input),
            None => output = String::from("Preferred profile does not exist")
        }
    } else {
        output = profiles.values().fold(input.to_string(), |s, p| p.apply(&s));
    }

    println!("{}", output);
}

#[cfg(test)]
mod tests {
    use crate::template::*;
    use std::path::PathBuf;

    #[test]
    fn raw_template() {
        let mut profile = test_profile().add_match(Template::raw("h", "hello"))
            .add_match(Template::raw("tem", "tempo"))
            .add_match(Template::date_time(":now", "%Y-%m-%d %H:%M:%S"));
        println!("{}", profile.apply("now is :now"));
        assert_eq!(profile.apply("h, tem"), "hello, tempo");
    }

    fn test_profile() -> Profile {
        Profile::new("test", PathBuf::from("test.txt"), vec![])
    }
}
