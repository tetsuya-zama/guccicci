extern crate guccicci;

use std::env;
use std::fs;
use guccicci::domain::TeamsCreationSetting;
use guccicci::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("argument setting file name is required");
    }

    let setting_filename = &args[1];
    let setting_str = fs::read_to_string(setting_filename).unwrap();
    let setting: TeamsCreationSetting = toml::from_str(&setting_str).unwrap();

    let res = run(setting).unwrap();
    print!("{}", toml::to_string_pretty(&res).unwrap());
}
