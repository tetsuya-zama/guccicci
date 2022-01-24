pub mod domain;
pub mod strategy;

use anyhow::Result;
use domain::{Teams, TeamsCreationSetting};
use strategy::ShuffleStrategies;

pub fn run(setting: TeamsCreationSetting ) -> Result<Teams> {
    let teams = Teams::create(setting, &ShuffleStrategies::RandomShuffle)?;

    Ok(teams)
}

