mod config;
mod template;

use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "0.1", author = "Yucklys <yucklys687@outlook.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {}

pub fn run_cli() {
    let opts: Opts = Opts::parse();
    println!("Hello, world");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
