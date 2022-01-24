use serde::{Deserialize, Serialize};
use anyhow::{Result};
use thiserror::{Error};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Person {
    pub name: String
}

#[derive(Debug, Serialize)]
pub struct Team {
    pub leader: Person,
    pub member: Vec<Person>
}

impl Team {
    pub fn new(leader: Person) -> Team {
        Team {
            leader,
            member: Vec::new()
        }
    }

    pub fn create_by_leader_candidates(mut leader_candidates: Vec<Person>, num_of_teams: u8) -> (Vec<Team>, Vec<Person>) {
        let mut res: Vec<Team> = Vec::new();

        while res.len() < num_of_teams.into() {
            let leader = leader_candidates.pop().unwrap();
            res.push(Team::new(leader));
        }

        (res, leader_candidates)
    }

    pub fn assign(&mut self, new_member: Person) {
        self.member.push(new_member);
    }
}

pub trait VecShuffleStrategy {
    fn shuffle<T>(&self, vec: &mut Vec<T>) -> Result<()>;
}

#[derive(Debug, Serialize)]
pub struct Teams {
    team: Vec<Team>
}

impl Teams {
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

    pub fn borrow_vec(&self) -> &Vec<Team> {
        &self.team
    }
}

#[derive(Debug, Deserialize)]
pub struct Attendee {
    person: Person,
    leader: Option<bool>
}

impl Attendee {
    pub fn is_leader(&self) -> bool {
        self.leader.unwrap_or(false)
    }
}

#[derive(Debug,Error)]
pub enum TeamsCreationSettingError {
    #[error("num_of_teams must be more than zero.")]
    NumOfTeamsZero,
    #[error("num of leader candidates({0}) must be equal or grater than num of teams({1})")]
    LeadersLack(u8,u8)
}

#[derive(Debug, Deserialize)]
pub struct  TeamsCreationSetting {
    attendees: Vec<Attendee>,
    num_of_teams: u8,
    flat: Option<bool>
}

impl TeamsCreationSetting {
    pub fn is_flat(&self) -> bool {
        self.flat.unwrap_or(false)
    }

    pub fn leader_candidates(&self) -> Vec<&Person> {
        if self.is_flat() {
            self.all_people()
        }else{
            self.attendees.iter().filter(|a| a.is_leader()).map(|a| &a.person).collect()
        }
    }

    pub fn normal_attendees(&self) -> Vec<&Person> {
        if self.is_flat() {
            Vec::new()
        }else{
            self.attendees.iter().filter(|a| !a.is_leader()).map(|a| &a.person).collect()
        }
    }

    pub fn all_people(&self) -> Vec<&Person> {
        self.attendees.iter().map(|a| &a.person).collect()
    }

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

    #[test]
    fn create_team_by_leader() {
        let team = Team::new(Person{name: "A".to_string()});
        
        assert_eq!(team.leader.name, "A".to_string());
        assert_eq!(team.member.len(), 0);
    }

    #[test]
    fn assign_member_to_team() {
        let mut team = Team::new(Person{name: "A".to_string()});

        team.assign(Person{name: "B".to_string()});
        team.assign(Person{name: "C".to_string()});

        assert_eq!(team.member.len(), 2);

    }

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