use crate::players::{Candidate, Direction, PlayerPool, Players};
use crate::teams::Teams;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;

pub struct Mathmaking<'a> {
    players: &'a Players,
    tolerance: u32,
    total_sr: i32,
    total_count: usize,
    sec_roles: bool,
    teams: Teams,
    pool: PlayerPool,
    balanced: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BalancerResult {
    pub teams: Teams,
    pub leftovers: PlayerPool,
}

impl<'a> Mathmaking<'a> {
    pub fn new(players: &'a Players, tolerance: u32) -> Mathmaking {
        Mathmaking {
            players,
            tolerance,
            total_sr: 0,
            total_count: 0,
            sec_roles: false,
            teams: Teams::default(),
            balanced: Vec::default(),
            pool: PlayerPool::default(),
        }
    }

    pub fn balance_players(&mut self) {
        self.init_teams();
        self.distribute_squires();
        self.init_pool();
        self.distribute_leutenants();
        self.distribute_ensigns();
        self.distribute_fillers();
        self.distribute_remaining();
    }

    pub fn result(self) -> BalancerResult {
        BalancerResult::new(self.teams, self.pool)
    }

    fn preserve_players(&mut self, players: &PlayerPool) {
        self.balanced.extend(players.collect_ids().into_iter())
    }

    fn init_pool(&mut self) {
        self.players.feed(&mut self.pool, &self.balanced);
        self.pool.sort_by_rank(Direction::ASC);
    }

    fn init_teams(&mut self) {
        let mut captains = self.players.get_captains();
        captains.sort_by_rank(Direction::DESC);
        self.preserve_players(&captains);

        self.teams = Teams::from(captains);
    }

    fn distribute_remaining(&mut self) {
        self.update();
        self.pool.shuffle();
        self.teams.sort(Direction::ASC);

        self.sort_remaining(1);
    }

    fn distribute_squires(&mut self) {
        let mut squires = self.players.get_squires();
        squires.sort_by_rank(Direction::DESC);
        self.preserve_players(&squires);

        self.teams.distribute_squires_to_dps(&mut squires);
        self.teams.distribute_squires(&mut squires);
    }

    fn distribute_leutenants(&mut self) {
        self.teams.distribute_leutenants(&mut self.pool);
    }

    fn distribute_ensigns(&mut self) {
        self.teams.update();
        self.teams.sort(Direction::ASC);
        self.pool.sort_by_rank(Direction::ASC);
        self.teams.distribute_ensigns(&mut self.pool);
    }

    fn distribute_fillers(&mut self) {
        self.teams.update();
        self.teams.sort(Direction::DESC);
        let avg = self.get_players_average();

        console::log_2(&JsValue::from_str("Players Average: "), &JsValue::from(avg));

        self.pool.sort_by_rank(Direction::DESC);
        self.teams
            .distribute_fillers(&mut self.pool, self.tolerance, avg);
    }

    fn update(&mut self) {
        self.teams.update();
        let (total_sr, total_count) = self.teams.get_stats();

        self.total_sr = total_sr;
        self.total_count = total_count;
    }

    fn sort_remaining(&mut self, delta: usize) {
        let added_players = self.fit_players();

        if added_players == 0 && delta == 0 && !self.sec_roles && self.pool.size() > 0 {
            self.sec_roles = true;
            self.sort_remaining(added_players);
        } else if self.pool.size() > 0 && (added_players > 0 || !self.sec_roles) {
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
                self.total_sr += sr;
                self.total_count += 1;
                added_players += 1;
            } else {
                leftovers.add_candidate(candidate);
            }
        }

        self.pool = leftovers;

        added_players
    }

    fn fit_player(&mut self, candidate: &Candidate) -> Option<i32> {
        let roles_count = if !self.sec_roles {
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
            let new_average = (self.total_sr + player_sr) as f32 / (self.total_count + 1) as f32;
            if let Some(team) =
                self.teams
                    .fit_player(player_sr, new_average, self.tolerance, target_role)
            {
                team.add_player(candidate, target_role);
                team.update();

                return Some(player_sr);
            }
        }

        None
    }

    fn get_players_average(&self) -> i32 {
        let (total_sr, total_count) = self.teams.get_stats();
        self.pool.get_primary_average(total_sr, total_count)
    }
}

impl BalancerResult {
    fn new(teams: Teams, leftovers: PlayerPool) -> BalancerResult {
        BalancerResult { teams, leftovers }
    }
}
