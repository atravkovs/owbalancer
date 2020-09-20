use crate::players::{Candidate, Direction, PlayerPool, Players};
use crate::teams::Teams;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub total_sr: i32,
    pub tolerance: u32,
    pub sec_roles: bool,
    pub limiter_max: i32,
    pub total_count: usize,
    pub rank_limiter: bool,
    pub players_average: i32,
    pub duplicate_roles: bool,
    pub increase_players_count: bool,
}

pub struct Mathmaking<'a> {
    teams: Teams,
    config: Config,
    pool: PlayerPool,
    players: &'a Players,
    balanced: Vec<String>,
    reserve_pool: PlayerPool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BalancerResult {
    pub teams: Teams,
    pub leftovers: PlayerPool,
}

impl<'a> Mathmaking<'a> {
    pub fn new(
        players: &'a Players,
        tolerance: u32,
        rank_limiter: bool,
        duplicate_roles: bool,
    ) -> Mathmaking {
        let config = Config::new(tolerance, rank_limiter, duplicate_roles);

        Mathmaking {
            config,
            players,
            teams: Teams::default(),
            balanced: Vec::default(),
            pool: PlayerPool::default(),
            reserve_pool: PlayerPool::default(),
        }
    }

    pub fn balance_players(&mut self) {
        self.init_teams();
        self.distribute_squires();
        self.init_pool(false);
        self.distribute_leutenants();
        self.distribute_ensigns();
        self.distribute_fillers();
        self.distribute_remaining();
        self.swap_steal();
    }

    pub fn balance_half(&mut self) {
        self.init_teams();
        self.distribute_squires();
        self.init_pool(false);
        self.distribute_leutenants();
        self.distribute_ensigns();
        self.teams.sort(Direction::ASC);
    }

    pub fn balance_remaining(&mut self) {
        self.init_pool(true);
        self.distribute_fillers();
        self.distribute_remaining();
        self.swap_steal();
    }

    pub fn result(self) -> BalancerResult {
        BalancerResult::new(self.teams, self.pool)
    }

    pub fn add_reserve(&mut self, reserve: Vec<String>) {
        self.balanced = reserve;
    }

    pub fn add_teams(&mut self, teams: Teams) {
        self.teams = teams;
    }

    fn preserve_players(&mut self, players: &PlayerPool) {
        self.balanced.extend(players.collect_ids().into_iter())
    }

    fn init_pool(&mut self, invert: bool) {
        self.players.feed(&mut self.pool, &self.balanced, invert);
        self.pool.sort_by_rank(Direction::ASC);
        self.reserve_pool = self.pool.clone();
    }

    fn init_teams(&mut self) {
        let mut captains = self.players.get_captains();
        captains.sort_by_rank(Direction::ASC);
        self.preserve_players(&captains);

        self.teams = Teams::from(captains);
    }

    fn swap_steal(&mut self) {
        self.teams.update();
        let not_complete_teams = self.teams.get_not_complete();

        for id in not_complete_teams {
            let team = self.teams.get(id);
            let role = team.get_missing_role();
            if role.is_none() {
                continue;
            }

            let role = role.unwrap();
            let role_clone = role.clone();
            let range = team.get_range(&self.config);
            let find_replacement = self.pool.distribute_replacement(
                role,
                range,
                &self.teams,
                &self.reserve_pool,
                &team.clone(),
                &self.config,
            );

            if let Some((team_id, replacement_id, leftover)) = find_replacement {
                let replacement_team = self.teams.get(team_id);
                let replacement_member = replacement_team.members.get(replacement_id).unwrap();
                let replacement = self
                    .reserve_pool
                    .0
                    .iter()
                    .find(|candidate| candidate.uuid.as_str() == replacement_member.uuid.as_str())
                    .unwrap();

                let team = self.teams.get_mut(id);

                team.add_player(
                    &replacement,
                    replacement.roles.get_by_simple(&role_clone).unwrap(),
                );
                team.update();

                console::log_3(
                    &JsValue::from_str("Moved"),
                    &JsValue::from_str(team.name.as_str()),
                    &JsValue::from_str(replacement.name.as_str()),
                );

                let team = self.teams.get_mut(team_id);

                let replacement_member = team.members.remove(replacement_id);

                let pos = self
                    .pool
                    .0
                    .iter()
                    .position(|c| c.uuid.as_str() == leftover.uuid.as_str());
                if let Some(index) = pos {
                    let candidate = self.pool.0.get(index).unwrap();
                    let add_role = candidate
                        .roles
                        .get_by_simple(&replacement_member.role)
                        .unwrap();

                    console::log_3(
                        &JsValue::from_str("Add"),
                        &JsValue::from_str(team.name.as_str()),
                        &JsValue::from_str(candidate.name.as_str()),
                    );

                    team.add_player(candidate, &add_role);
                    team.update();
                    self.config.total_count += 1;
                    self.config.total_sr += add_role.decompose().1;
                    self.pool.0.remove(index);
                }
            }
        }

        if self.config.rank_limiter {
            self.config.rank_limiter = false;
            self.swap_steal();
        }

        self.teams.sort(Direction::ASC);
    }

    fn distribute_remaining(&mut self) {
        self.update();
        self.pool.shuffle();
        self.teams.sort(Direction::ASC);

        self.sort_remaining(1);
    }

    fn distribute_squires(&mut self) {
        let mut squires = self.players.get_squires();
        squires.sort_by_rank(Direction::ASC);
        self.preserve_players(&squires);

        self.teams.distribute_squires_to_dps(&mut squires);
        self.teams.update();
        self.teams.sort(Direction::DESC);
        self.teams.distribute_squires(&mut squires);
    }

    fn distribute_leutenants(&mut self) {
        self.teams
            .distribute_leutenants(&mut self.pool, &self.config);
    }

    fn distribute_ensigns(&mut self) {
        self.teams.update();
        self.teams.sort(Direction::ASC);
        self.pool.sort_by_rank(Direction::ASC);
        self.teams.distribute_ensigns(&mut self.pool, &self.config);
    }

    fn distribute_fillers(&mut self) {
        self.teams.update();
        self.teams.sort(Direction::DESC);
        self.calculate_players_average();

        self.pool.sort_by_rank(Direction::DESC);
        self.teams.distribute_fillers(&mut self.pool, &self.config);
    }

    fn update(&mut self) {
        self.teams.update();
        let (total_sr, total_count) = self.teams.get_stats();

        self.config.total_sr = total_sr;
        self.config.total_count = total_count;
    }

    fn sort_remaining(&mut self, delta: usize) {
        let added_players = self.fit_players();

        if added_players == 0 && delta == 0 && !self.config.sec_roles && self.pool.size() > 0 {
            self.config.sec_roles = true;
            self.sort_remaining(added_players);
        } else if added_players == 0
            && delta == 0
            && self.config.duplicate_roles
            && self.pool.size() > 0
        {
            self.config.duplicate_roles = false;
            self.sort_remaining(added_players);
        } else if self.pool.size() > 0 && (added_players > 0 || !self.config.sec_roles) {
            self.sort_remaining(added_players);
        }
    }

    fn fit_players(&mut self) -> usize {
        let mut leftovers: PlayerPool = PlayerPool::default();
        let mut added_players = 0;
        let pool = &self.pool.0.clone();

        for candidate in pool {
            let fits = self.fit_player(candidate);
            if let Some(sr) = fits {
                self.config.total_sr += sr;
                self.config.total_count += 1;
                added_players += 1;
            } else {
                leftovers.add_candidate(candidate);
            }
        }

        self.pool = leftovers;

        added_players
    }

    fn fit_player(&mut self, candidate: &Candidate) -> Option<i32> {
        let roles_count = if !self.config.sec_roles {
            if candidate.roles_count() > 0 {
                1
            } else {
                0
            }
        } else {
            candidate.roles_count()
        };

        for role_index in 0..roles_count {
            let target_role = candidate.roles.get(role_index);

            let (_, player_sr) = target_role.decompose();
            let new_average =
                (self.config.total_sr + player_sr) as f32 / (self.config.total_count + 1) as f32;
            if let Some(team) =
                self.teams
                    .fit_player(player_sr, new_average, &self.config, target_role)
            {
                team.add_player(candidate, target_role);
                team.update();

                return Some(player_sr);
            }
        }

        None
    }

    fn calculate_players_average(&mut self) {
        let (total_sr, total_count) = self.teams.get_stats();
        self.config.players_average = self.pool.get_primary_average(total_sr, total_count);
    }
}

impl Config {
    fn new(tolerance: u32, rank_limiter: bool, duplicate_roles: bool) -> Config {
        Config {
            tolerance,
            rank_limiter,
            duplicate_roles,
            total_sr: 0,
            total_count: 0,
            sec_roles: false,
            limiter_max: 2500,
            players_average: 0,
            increase_players_count: false,
        }
    }
}

impl BalancerResult {
    fn new(teams: Teams, leftovers: PlayerPool) -> BalancerResult {
        BalancerResult { teams, leftovers }
    }
}
