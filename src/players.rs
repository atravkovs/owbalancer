use crate::roles::{Role, Roles, RolesFilter};
use crate::teams::{Team, Teams};
use rand::rngs::OsRng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub uuid: String,
    pub name: String,
    pub is_squire: bool,
    pub is_captain: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    pub classes: Classes,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub identity: Identity,
    pub stats: Stats,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClassType {
    pub rank: i32,
    pub priority: i16,
    pub primary: bool,
    pub secondary: bool,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Classes {
    pub dps: ClassType,
    pub tank: ClassType,
    pub support: ClassType,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Players(HashMap<String, Player>);

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Candidate {
    pub uuid: String,
    pub name: String,
    pub roles: Roles,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PlayerPool(pub Vec<Candidate>);

#[derive(PartialEq)]
pub enum Direction {
    ASC,
    DESC,
}

impl Players {
    pub fn get_captains(&self) -> PlayerPool {
        let players = self
            .0
            .iter()
            .filter_map(|(_, player)| {
                if player.identity.is_captain {
                    Some(Candidate::from(player))
                } else {
                    None
                }
            })
            .collect();

        PlayerPool(players)
    }

    pub fn get_squires(&self) -> PlayerPool {
        let players = self
            .0
            .iter()
            .filter_map(|(_, player)| {
                if player.identity.is_squire {
                    Some(Candidate::from(player))
                } else {
                    None
                }
            })
            .collect();

        PlayerPool(players)
    }

    pub fn feed(&self, pool: &mut PlayerPool, exclude: &Vec<String>) {
        for (_, player) in &self.0 {
            if !exclude.contains(&player.identity.uuid) {
                pool.add_player(player);
            }
        }
    }
}

impl Candidate {
    pub fn get_primary_role(&self) -> &Role {
        self.roles.get_primary()
    }

    pub fn roles_count(&self) -> usize {
        self.roles.count()
    }

    fn new(uuid: String, name: String, roles: Roles) -> Candidate {
        Candidate { uuid, name, roles }
    }
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl From<&Player> for Candidate {
    fn from(player: &Player) -> Self {
        Candidate::new(
            player.identity.uuid.clone(),
            player.identity.name.clone(),
            Roles::from(&player.stats.classes),
        )
    }
}

impl PlayerPool {
    pub fn sort_by_rank(&mut self, direction: Direction) {
        self.0.sort_by(|a, b| {
            let ordering = b.roles.get_primary_rank().cmp(&a.roles.get_primary_rank());

            if direction == Direction::DESC {
                return ordering.reverse();
            }

            ordering
        })
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn add_player(&mut self, player: &Player) {
        self.0.push(Candidate::from(player));
    }

    pub fn add_candidate(&mut self, candidate: &Candidate) {
        self.0.push(candidate.clone());
    }

    pub fn collect_ids(&self) -> Vec<String> {
        self.0
            .iter()
            .map(|candidate| candidate.uuid.clone())
            .collect()
    }

    pub fn filter_by_roles(&self, roles_filter: RolesFilter) -> Vec<&Candidate> {
        self.0
            .iter()
            .filter(|&candidate| roles_filter.has_same(candidate.roles.get_primary()))
            .collect()
    }

    pub fn dispose_of(&mut self, candidates: Vec<&Candidate>) {
        self.0.retain(|candidate| !candidates.contains(&candidate));
    }

    pub fn shuffle(&mut self) {
        self.0.shuffle(&mut OsRng::default());
    }

    pub fn distribute_leutenant(&mut self, teams: &mut Teams, offset: usize) -> usize {
        if offset >= self.0.len() {
            return offset;
        }
        let candidate = self.0.get(offset).unwrap().clone();

        if let Some(team) = teams.find_mate(&candidate, 2) {
            return self.add_player_to_team(team, &candidate, offset);
        }

        if let Some(team) = teams.find_team(2, candidate.get_primary_role()) {
            return self.add_player_to_team(team, &candidate, offset);
        }

        self.distribute_leutenant(teams, offset + 1)
    }

    pub fn distribute_ensign(&mut self, teams: &mut Teams, offset: usize) -> usize {
        if offset >= self.0.len() {
            return offset;
        }
        let candidate = self.0.get(offset).unwrap().clone();

        if let Some(team) = teams.find_perfect_ensign(&candidate) {
            return self.add_player_to_team(team, &candidate, offset);
        }

        if let Some(team) = teams.find_mate(&candidate, 3) {
            return self.add_player_to_team(team, &candidate, offset);
        }

        if let Some(team) = teams.find_team(3, candidate.get_primary_role()) {
            return self.add_player_to_team(team, &candidate, offset);
        }

        self.distribute_ensign(teams, offset + 1)
    }

    fn add_player_to_team(
        &mut self,
        team: &mut Team,
        candidate: &Candidate,
        offset: usize,
    ) -> usize {
        team.add_primary_player(candidate);
        self.0.remove(offset);

        offset
    }
}
