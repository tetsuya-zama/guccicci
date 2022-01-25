pub mod domain;
pub mod strategy;

use anyhow::Result;
use domain::{Teams, TeamsCreationSetting};
use strategy::ShuffleStrategies;

/// チーム作成を実行する
/// # Attributes
/// * `setting` - チーム作成設定
/// # Return
/// Ok(作成されたチーム)
pub fn run(setting: TeamsCreationSetting ) -> Result<Teams> {
    let teams = Teams::create(setting, &ShuffleStrategies::RandomShuffle)?;

    Ok(teams)
}

