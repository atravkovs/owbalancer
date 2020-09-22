use crate::matchmaking::Config;
use crate::roles::{Role, Roles, RolesFilter, SimpleRole};
use crate::teams::{Team, Teams};
use crate::AsjustSr;
use rand::rngs::OsRng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::MulAssign;

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

    pub fn feed(&self, pool: &mut PlayerPool, exclude: &Vec<String>, invert: bool) {
        for (_, player) in &self.0 {
            let mut pass = exclude.contains(&player.identity.uuid);

            if !invert {
                pass = !pass;
            }

            if pass {
                pool.add_player(player);
            }
        }
    }

    pub fn adjust_sr(&mut self, adjust: AsjustSr) {
        if !adjust.is_enabled {
            return;
        }

        for (_, player) in &mut self.0 {
            if player.stats.classes.dps.is_active {
                player.stats.classes.dps.rank = (player.stats.classes.dps.rank as f32
                    * (adjust.dps as f32 / 100.0))
                    .floor() as i32;
            }

            if player.stats.classes.support.is_active {
                player.stats.classes.support.rank = (player.stats.classes.support.rank as f32
                    * (adjust.support as f32 / 100.0))
                    .floor() as i32;
            }

            if player.stats.classes.tank.is_active {
                player.stats.classes.tank.rank = (player.stats.classes.tank.rank as f32
                    * (adjust.tank as f32 / 100.0))
                    .floor() as i32;
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

    pub fn filter_by_roles(&self, roles_filter: RolesFilter) -> Vec<usize> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(index, candidate)| {
                if roles_filter.has_same(candidate.roles.get_primary()) {
                    Some(index)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn dispose_of(&mut self, candidates: Vec<usize>) {
        self.0 = self
            .0
            .iter_mut()
            .enumerate()
            .filter_map(|(id, candidate)| {
                if candidates.contains(&id) {
                    None
                } else {
                    Some(candidate.clone())
                }
            })
            .collect()
    }

    pub fn shuffle(&mut self) {
        self.0.shuffle(&mut OsRng::default());
    }

    pub fn distribute_leutenant(
        &mut self,
        teams: &mut Teams,
        offset: usize,
        config: &Config,
    ) -> usize {
        if offset >= self.0.len() {
            return offset;
        }
        let candidate = self.0.get(offset).unwrap().clone();

        if let Some(team) = teams.find_mate(&candidate, 2, config) {
            return self.add_player_to_team(team, &candidate, offset);
        }

        if let Some(team) = teams.find_team(2, candidate.get_primary_role(), config) {
            return self.add_player_to_team(team, &candidate, offset);
        }

        self.distribute_leutenant(teams, offset + 1, config)
    }

    pub fn distribute_ensign(
        &mut self,
        teams: &mut Teams,
        offset: usize,
        config: &Config,
    ) -> usize {
        if offset >= self.0.len() {
            return offset;
        }
        let candidate = self.0.get(offset).unwrap().clone();

        if let Some(team) = teams.find_perfect_ensign(&candidate, config) {
            return self.add_player_to_team(team, &candidate, offset);
        }

        if let Some(team) = teams.find_mate(&candidate, 3, config) {
            return self.add_player_to_team(team, &candidate, offset);
        }

        if let Some(team) = teams.find_team(3, candidate.get_primary_role(), config) {
            return self.add_player_to_team(team, &candidate, offset);
        }

        self.distribute_ensign(teams, offset + 1, config)
    }

    pub fn distribute_filler(&mut self, team: &mut Team, config: &Config) {
        let range = team.get_range(config);
        let clonned = self.clone();
        let find_candidate = clonned.filter_range(range, team, config);

        if let Some(candidate) = find_candidate {
            team.add_primary_player(candidate);
            self.remove_candidate(candidate);
        } else {
            for candidate in &clonned.0 {
                for i in 1..candidate.roles_count() {
                    let role = candidate.roles.get(i);

                    if role.is_in_range(range) && role.fits_team(team, config) {
                        team.add_player(candidate, role);
                        self.remove_candidate(candidate);
                        return;
                    }
                }
            }
        }
    }

    pub fn distribute_replacement(
        &self,
        role: SimpleRole,
        range: (i32, i32),
        teams: &Teams,
        db: &PlayerPool,
        target_team: &Team,
        config: &Config,
    ) -> Option<(usize, usize, &Candidate)> {
        for leftover in &self.0 {
            let lost = teams.replace_leftover(leftover, &role, range, db, target_team, config);
            if let Some(replacement) = lost {
                return Some((replacement.0, replacement.1, leftover));
            }
        }

        None
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

    pub fn get_by_id(&self, uuid: String) -> Option<&Candidate> {
        self.0.iter().find(|&candidate| candidate.uuid == uuid)
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

    fn filter_range(&self, range: (i32, i32), team: &Team, config: &Config) -> Option<&Candidate> {
        self.0.iter().find(|&candidate| {
            let role = candidate.get_primary_role();
            role.is_in_range(range) && role.fits_team(team, config)
        })
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
