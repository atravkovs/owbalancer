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

        if let Some(team) = teams.find_mate(&candidate, 4) {
            return self.add_player_to_team(team, &candidate, offset);
        }

        if let Some(team) = teams.find_team(4, candidate.get_primary_role()) {
            return self.add_player_to_team(team, &candidate, offset);
        }

        self.distribute_ensign(teams, offset + 1)
    }

    pub fn distribute_filler(&mut self, team: &mut Team, tolerance: u32, players_average: i32) {
        let range = Self::get_range(
            team.total_sr,
            team.members_count(),
            tolerance,
            players_average,
        );
        let clonned = self.clone();
        let pool = clonned.filter_range(range);

        let find_candidate = pool
            .iter()
            .find(|&candidate| candidate.get_primary_role().fits_team(team));

        if let Some(candidate) = find_candidate {
            team.add_primary_player(candidate);
            self.remove_candidate(candidate);
        } else {
            for candidate in pool {
                for i in 1..candidate.roles_count() {
                    let role = candidate.roles.get(i);
                    if role.fits_team(team)
                        && (role.is_same(&team.get_captain().role)
                            || role.is_same(&team.get_leutenant().role))
                    {
                        team.add_player(candidate, role);
                        self.remove_candidate(candidate);
                    }
                }
            }
        }
    }

    pub fn get_primary_average(&self, teams_sr: i32, teams_count: usize) -> i32 {
        let sum: i32 = self
            .0
            .iter()
            .map(|candidate| candidate.get_primary_role().decompose().1)
            .sum();
        let count = self.0.len();

        let mutual_sum = sum + teams_sr;
        let mutual_count = count + teams_count;

        (mutual_sum as f32 / mutual_count as f32).floor() as i32
    }

    fn remove_candidate(&mut self, candidate: &Candidate) {
        let find = self
            .0
            .iter()
            .position(|stored_candidate| stored_candidate == candidate);

        if let Some(index) = find {
            self.0.remove(index);
        }
    }

    fn filter_range(&self, range: (i32, i32)) -> Vec<&Candidate> {
        self.0
            .iter()
            .filter(|&candidate| {
                let rank = candidate.roles.get_primary_rank();
                rank >= range.0 && rank <= range.1
            })
            .collect()
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

    fn get_range(
        team_sr: i32,
        team_count: usize,
        tolerance: u32,
        players_average: i32,
    ) -> (i32, i32) {
        let players_count = 6;
        let tolerance_range = tolerance * players_count;
        let target_sr = (players_average * (team_count as i32 + 1)) - team_sr;
        let min_sr = target_sr - tolerance_range as i32;
        let max_sr = target_sr + tolerance_range as i32;

        (min_sr, max_sr)
    }
}
