use serde::{Deserialize, Serialize};
use anyhow::{Result};
use thiserror::{Error};


/// 人物を表すStruct
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Person {
    /// 人物の名前
    pub name: String
}

/// チームを表すStruct
#[derive(Debug, Serialize)]
pub struct Team {
    /// チームリーダー
    pub leader: Person,
    /// チームメンバー
    pub member: Vec<Person>
}

impl Team {
    /// 新しいチームを作成する
    /// # Attributes
    /// * `leader` - チームのリーダー
    /// 
    /// # Returns
    /// `leader`がリーダー`Team`のインスタンス
    pub fn new(leader: Person) -> Team {
        Team {
            leader,
            member: Vec::new()
        }
    }

    /// リーダー候補者とチーム数からチームを複数作成する
    /// 
    /// # Attributes
    /// * `leader_candidates` - リーダー候補者
    /// * `num_of_teams` - チーム数
    /// 
    /// # Returns
    /// (作成されたチームのVec, リーダー候補者のうちリーダーになっていない人のVec)のタプル
    pub fn create_by_leader_candidates(mut leader_candidates: Vec<Person>, num_of_teams: u8) -> (Vec<Team>, Vec<Person>) {
        let mut res: Vec<Team> = Vec::new();

        while res.len() < num_of_teams.into() {
            let leader = leader_candidates.pop().unwrap();
            res.push(Team::new(leader));
        }

        (res, leader_candidates)
    }

    /// チームにメンバーをアサインする
    /// # Attributes
    /// * `new_member` - アサインしたいメンバー
    pub fn assign(&mut self, new_member: Person) {
        self.member.push(new_member);
    }
}

/// 配列のシャッフルの仕方を定義するStrategy
pub trait VecShuffleStrategy {
    /// `vec`に与えられたVec<T>をシャッフルする。
    /// `vec`を破壊するメソッドである点注意
    /// # Attributes
    /// * `vec` - シャッフルしたい対象のVec
    fn shuffle<T>(&self, vec: &mut Vec<T>) -> Result<()>;
}

/// `Team`の集約
#[derive(Debug, Serialize)]
pub struct Teams {
    /// `Team`のリスト
    team: Vec<Team>
}

impl Teams {
    /// 設定値から`Team`の集約を作成する
    /// # Attributes
    /// * `setting` - ユーザーから与えられた設定値
    /// * `shuffle_strategy` - `Vec`のshuffleの仕方
    /// 
    /// # Returns
    /// Result<作成された`Teams`, anyhow::Error>
    pub fn create(setting: TeamsCreationSetting, shuffle_strategy: &impl VecShuffleStrategy) -> Result<Teams> {
        setting.validate()?;

        let mut leader_candidates: Vec<Person> = setting.leader_candidates().iter().map(|p| p.clone()).cloned().collect();
        shuffle_strategy.shuffle(&mut leader_candidates)?;

        let (mut teams_vec, mut rest) = Team::create_by_leader_candidates(leader_candidates, setting.num_of_teams);
        let mut normal_attendees: Vec<Person> = setting.normal_attendees().iter().map(|p| p.clone()).cloned().collect();
        rest.append(&mut normal_attendees);

        shuffle_strategy.shuffle(&mut rest)?;

        while !rest.is_empty() {
            for team in &mut teams_vec {
                if let Some(m) = rest.pop(){
                    team.assign(m);
                }else{
                    break;
                }
            }
        }

        Ok(Teams {team:teams_vec})
    }

    /// Vecとして借用する
    /// # Returns
    /// `team`のリスト
    pub fn borrow_vec(&self) -> &Vec<Team> {
        &self.team
    }
}

/// 参加者を表すstruct
#[derive(Debug, Deserialize)]
pub struct Attendee {
    /// 人物
    person: Person,
    /// リーダになりうるか
    leader: Option<bool>
}

impl Attendee {
    /// リーダになりうるかを返す
    /// # Returns
    /// リーダー候補であればtrue
    pub fn is_leader(&self) -> bool {
        self.leader.unwrap_or(false)
    }
}

/// チーム作成設定に関するエラー
#[derive(Debug,Error)]
pub enum TeamsCreationSettingError {
    /// チーム数にゼロが設定されている
    #[error("num_of_teams must be more than zero.")]
    NumOfTeamsZero,
    /// チーム数に対してリーダー候補が少なすぎる
    #[error("num of leader candidates({0}) must be equal or grater than num of teams({1})")]
    LeadersLack(u8,u8)
}

/// チーム作成設定
#[derive(Debug, Deserialize)]
pub struct  TeamsCreationSetting {
    /// 出席者のリスト
    attendees: Vec<Attendee>,
    /// チーム数
    num_of_teams: u8,
    /// フラットフラグ
    /// trueの場合はAttendeeのis_leaderの値を無視して全員リーダー候補とみなす
    flat: Option<bool>
}

impl TeamsCreationSetting {
    /// フラットフラグの値を返す
    /// # Returns
    /// 全員をリーダー候補とみなす場合はtrue
    pub fn is_flat(&self) -> bool {
        self.flat.unwrap_or(false)
    }

    /// リーダー候補の参加者を返す
    /// # Returns
    /// リーダー候補の`Person`のリスト
    pub fn leader_candidates(&self) -> Vec<&Person> {
        if self.is_flat() {
            self.all_people()
        }else{
            self.attendees.iter().filter(|a| a.is_leader()).map(|a| &a.person).collect()
        }
    }

    /// リーダー候補以外の参加者を返す
    /// # Returns
    /// リーダー候補以外の`Person`のリスト
    pub fn normal_attendees(&self) -> Vec<&Person> {
        if self.is_flat() {
            Vec::new()
        }else{
            self.attendees.iter().filter(|a| !a.is_leader()).map(|a| &a.person).collect()
        }
    }

    /// 全ての参加者を返す
    /// # Returns
    /// 全ての参加者の`Person`のリスト
    pub fn all_people(&self) -> Vec<&Person> {
        self.attendees.iter().map(|a| &a.person).collect()
    }

    /// チーム作成設定を検証する
    /// # Returns
    /// 検証エラーがなければOk<()>, エラーがあればErr<TeamsCreationSettingError>
    pub fn validate(&self) -> Result<(), TeamsCreationSettingError> {
        let num_of_leader_candidates = self.leader_candidates().len();

        if self.num_of_teams == 0 {
            Err(TeamsCreationSettingError::NumOfTeamsZero)?
        } else if  num_of_leader_candidates.lt(&self.num_of_teams.into()) {
            Err(TeamsCreationSettingError::LeadersLack(
                u8::try_from(num_of_leader_candidates).unwrap(), 
                u8::from(self.num_of_teams)
            ))?
        }else {
            Ok(())
        }
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    /// Attendee#is_leaderのテスト
    /// Noneであればデフォルト値であるfalseを返し、
    /// Someであればその値を返す
    #[test]
    fn attendee_is_leader() {
        let attendee1 = Attendee{
            person: Person{name: "A".to_string()},
            leader: None
        };

        let attendee2 = Attendee{
            person: Person{name: "B".to_string()},
            leader: Some(false)
        };

        let attendee3 = Attendee{
            person: Person{name: "C".to_string()},
            leader: Some(true)
        };

        assert_eq!(attendee1.is_leader(), false);
        assert_eq!(attendee2.is_leader(), false);
        assert_eq!(attendee3.is_leader(), true);
    }

    /// TeamsCreationSetting#is_flatのテスト
    /// Noneであればデフォルト値であるfalseを返し、
    /// Someであればその値を返す
    #[test]
    fn setting_is_flat() {
        let setting1 = TeamsCreationSetting{
            attendees: vec![
                Attendee{person: Person{name: String::from("A")}, leader: Some(true)},
                Attendee{person: Person{name: String::from("B")}, leader: Some(true)},
                Attendee{person: Person{name: String::from("C")}, leader: Some(false)},
                Attendee{person: Person{name: String::from("D")}, leader: Some(false)},
            ],
            num_of_teams: 2,
            flat: Some(true)
        };
        let setting2 = TeamsCreationSetting{
            attendees: vec![
                Attendee{person: Person{name: String::from("A")}, leader: Some(true)},
                Attendee{person: Person{name: String::from("B")}, leader: Some(true)},
                Attendee{person: Person{name: String::from("C")}, leader: Some(false)},
                Attendee{person: Person{name: String::from("D")}, leader: Some(false)},
            ],
            num_of_teams: 2,
            flat: Some(false)
        };
        let setting3 = TeamsCreationSetting{
            attendees: vec![
                Attendee{person: Person{name: String::from("A")}, leader: Some(true)},
                Attendee{person: Person{name: String::from("B")}, leader: Some(true)},
                Attendee{person: Person{name: String::from("C")}, leader: Some(false)},
                Attendee{person: Person{name: String::from("D")}, leader: Some(false)},
            ],
            num_of_teams: 2,
            flat: None
        };

        assert_eq!(setting1.is_flat(), true);
        assert_eq!(setting2.is_flat(), false);
        assert_eq!(setting3.is_flat(), false);
    }

    /// TeamsCreationSetting#validateのテスト
    /// Okの場合
    #[test]
    fn setting_validation_ok() {
        let setting = TeamsCreationSetting{
            attendees: vec![
                Attendee{person: Person{name: String::from("A")}, leader: Some(true)},
                Attendee{person: Person{name: String::from("B")}, leader: Some(true)},
                Attendee{person: Person{name: String::from("C")}, leader: Some(false)},
                Attendee{person: Person{name: String::from("D")}, leader: Some(false)},
            ],
            num_of_teams: 2,
            flat: None
        };
        
        match setting.validate() {
            Ok(_) => assert!(true),
            Err(e) => assert!(false, "Validation error occured, {}", e)
        }
    }

    /// TeamsCreationSetting#validateのテスト
    /// チーム数が0の場合はバリデーションエラー
    #[test]
    fn setting_validation_zero_teams() {
        let setting = TeamsCreationSetting{
            attendees: vec![
                Attendee{person: Person{name: String::from("A")}, leader: Some(true)},
                Attendee{person: Person{name: String::from("B")}, leader: Some(true)},
                Attendee{person: Person{name: String::from("C")}, leader: Some(false)},
                Attendee{person: Person{name: String::from("D")}, leader: Some(false)},
            ],
            num_of_teams: 0,
            flat: None
        };

        match setting.validate() {
            Ok(_) => assert!(false, "validation passed unexpectedly"),
            Err(e) => {
                match e {
                    TeamsCreationSettingError::NumOfTeamsZero => assert!(true),
                    TeamsCreationSettingError::LeadersLack(_,__) => assert!(false, "Unexpected error, {}", e)
                }
            }
        }
    }

    /// TeamsCreationSetting#validateのテスト
    /// チーム数よりもリーダー候補者が少なければバリデーションエラー
    #[test]
    fn setting_validation_leaders_lack() {
        let setting = TeamsCreationSetting{
            attendees: vec![
                Attendee{person: Person{name: String::from("A")}, leader: Some(true)},
                Attendee{person: Person{name: String::from("B")}, leader: Some(true)},
                Attendee{person: Person{name: String::from("C")}, leader: Some(false)},
                Attendee{person: Person{name: String::from("D")}, leader: Some(false)},
            ],
            num_of_teams: 3,
            flat: None
        };

        match setting.validate() {
            Ok(_) => assert!(false, "validation passed unexpectedly"),
            Err(e) => {
                match e {
                    TeamsCreationSettingError::NumOfTeamsZero => assert!(false, "Unexpected error, {}", e),
                    TeamsCreationSettingError::LeadersLack(_,__) => assert!(true)
                }
            }
        }
    }

    /// TeamsCreationSetting#leader_candidates, TeamsCreationSetting#normal_attendees, TeamsCreationSetting#all_peopleのテスト
    /// is_flatがfalseであればそれぞれリーダー候補者、リーダ候補者以外、全ての参加者をそのまま返す
    #[test]
    fn attendees_no_flat() {
        let setting = TeamsCreationSetting{
            attendees: vec![
                Attendee{person: Person{name: String::from("A")}, leader: Some(true)},
                Attendee{person: Person{name: String::from("B")}, leader: Some(true)},
                Attendee{person: Person{name: String::from("C")}, leader: Some(false)},
                Attendee{person: Person{name: String::from("D")}, leader: Some(false)},
            ],
            num_of_teams: 2,
            flat: None
        };

        assert_eq!(setting.leader_candidates().len(), 2);
        assert_eq!(setting.normal_attendees().len(), 2);
        assert_eq!(setting.all_people().len(), 4);
    }

    /// TeamsCreationSetting#leader_candidates, TeamsCreationSetting#normal_attendees, TeamsCreationSetting#all_peopleのテスト
    /// is_flatがtrueであればリーダー候補者 = 全ての参加者, リーダ候補者以外 = []となる
    #[test]
    fn attendees_flat() {
        let setting = TeamsCreationSetting{
            attendees: vec![
                Attendee{person: Person{name: String::from("A")}, leader: Some(true)},
                Attendee{person: Person{name: String::from("B")}, leader: Some(true)},
                Attendee{person: Person{name: String::from("C")}, leader: Some(false)},
                Attendee{person: Person{name: String::from("D")}, leader: Some(false)},
            ],
            num_of_teams: 2,
            flat: Some(true)
        };

        assert_eq!(setting.leader_candidates().len(), 4);
        assert_eq!(setting.normal_attendees().len(), 0);
        assert_eq!(setting.all_people().len(), 4);
    }

    /// Team#newのテスト
    /// リーダーを指定して`Team`のインスタンスを作成する
    #[test]
    fn create_team_by_leader() {
        let team = Team::new(Person{name: "A".to_string()});
        
        assert_eq!(team.leader.name, "A".to_string());
        assert_eq!(team.member.len(), 0);
    }

    /// Team#assignのテスト
    /// `Team`のインスタンスに対してリーダー以外のメンバーを追加する
    #[test]
    fn assign_member_to_team() {
        let mut team = Team::new(Person{name: "A".to_string()});

        team.assign(Person{name: "B".to_string()});
        team.assign(Person{name: "C".to_string()});

        assert_eq!(team.member.len(), 2);

    }

    /// Team#create_by_leader_candidatesのテスト
    /// リーダー候補者とチーム数を渡して複数の`Team`を作成する
    /// リーダー候補者数 > チーム数の場合はリーダーにアサインされなかったリーダー候補者を合わせて返す
    #[test]
    fn create_team_by_leader_candidates() {
        let leader_candidates = vec![
            Person{name: "A".to_string()}, 
            Person{name: "B".to_string()}, 
            Person{name: "C".to_string()}
        ];

        let (teams, rest) = Team::create_by_leader_candidates(leader_candidates, 2);

        assert_eq!(teams.len(), 2);
        assert_eq!(rest.len(), 1);
    }

    /// Teams#createのテスト
    /// `TeamsCreationSetting`の内容をもとに複数のチームを作成し、リーダーとリーダー以外のメンバーを設定して返す
    #[test]
    fn create_teams_by_setting() {
        let setting = TeamsCreationSetting{
            attendees: vec![
                Attendee{person: Person{name: String::from("A")}, leader: Some(true)},
                Attendee{person: Person{name: String::from("B")}, leader: Some(true)},
                Attendee{person: Person{name: String::from("C")}, leader: Some(false)},
                Attendee{person: Person{name: String::from("D")}, leader: Some(false)},
                Attendee{person: Person{name: String::from("E")}, leader: Some(true)},
            ],
            num_of_teams: 2,
            flat: Some(false)
        };

        let teams = Teams::create(setting, &crate::strategy::ShuffleStrategies::RandomShuffle).unwrap();

        assert_eq!(teams.team.len(), 2);

        let team1 = &teams.team[0];
        let team2 = &teams.team[1];

        assert_eq!(team1.member.len(), 2); //1 leader, 2 members
        assert_eq!(team2.member.len(), 1); //1 leader, 1 memberß

    }
}