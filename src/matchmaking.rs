use crate::players::{Candidate, Direction, PlayerPool, Players};
use crate::roles::SimpleRole;
use crate::teams::{Member, Team, Teams};
use crate::wasm_log;
use serde::{Deserialize, Serialize};
use std::cmp;

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
    pub rank_limiter2: bool,
    pub dispersion_minimizer: bool,
}

pub struct Mathmaking<'a> {
    teams: Teams,
    config: Config,
    pool: PlayerPool,
    players: &'a Players,
    balanced: Vec<String>,
    reserve_pool: PlayerPool,
    disable_type: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BalancerResult {
    pub teams: Teams,
    pub leftovers: PlayerPool,
    pub dispersion: i32,
    pub anchors: i32,
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
            disable_type: String::from("none"),
        }
    }

    pub fn set_disable_type(&mut self, disable_type: String) {
        self.disable_type = disable_type;
    }

    pub fn enable_dispersion_minimizer(&mut self) {
        self.config.dispersion_minimizer = true;
    }

    pub fn balance_players(&mut self) {
        self.log("Init");
        self.init_teams();
        self.log("Distribute squires");
        if self.disable_type.as_str() != "ex_caps" {
            self.distribute_squires();
        }
        self.log("Init pool");
        self.init_pool(false);

        self.log("Distribute leutenants");
        if self.disable_type.as_str() != "ex_caps" && self.disable_type.as_str() != "leut_ens" {
            self.distribute_leutenants();
        }

        self.log("Distribute ensigns");
        if self.disable_type.as_str() != "ex_caps"
            && self.disable_type.as_str() != "leut_ens"
            && self.disable_type.as_str() != "ens"
        {
            self.distribute_ensigns();
        }

        self.log("Distribute fillers");
        self.distribute_fillers();
        self.log("Distribute remaining");
        self.distribute_remaining();
        self.log("Swap Steal");
        self.swap_steal();
        self.log("Increase quiality");
        self.increase_quality();
        self.log("Minimize dispersion");
        self.minimize_dispersion();
        self.teams.sort(Direction::ASC);
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
        self.increase_quality();
        self.teams.sort(Direction::ASC);
    }

    pub fn result(self) -> BalancerResult {
        let dispersion = self.calculate_dispersion();
        let lows = self.teams.total_low_role_count(self.config.limiter_max);
        BalancerResult::new(self.teams, self.pool, dispersion, lows)
    }

    pub fn add_reserve(&mut self, reserve: Vec<String>) {
        self.balanced = reserve;
    }

    pub fn add_teams(&mut self, teams: Teams) {
        self.teams = teams;
    }

    fn calculate_dispersion(&self) -> i32 {
        let first = self.teams.0.first().unwrap();
        let last = self.teams.0.last().unwrap();
        let avg = self.config.total_sr / self.config.total_count as i32;

        let low_disp = (first.avg_sr as i32 - avg).abs();
        let high_disp = (last.avg_sr as i32 - avg).abs();

        cmp::max(low_disp, high_disp)
    }

    fn log(&self, message: &str) {
        wasm_log(String::from(message));
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

    fn minimize_dispersion(&mut self) {
        let not_complete_teams = self.teams.get_not_complete();
        if !self.config.dispersion_minimizer
            || (self.config.dispersion_minimizer && not_complete_teams.len() > 0)
        {
            return;
        }

        while let Some(swap) = self.try_minimize() {
            let first = self.teams.0[swap.0].members.remove(swap.1);
            let second = self.teams.0[swap.2].members.remove(swap.3);
            self.teams.0[swap.2].members.push(first);
            self.teams.0[swap.0].members.push(second);
            self.teams.0[swap.2].update();
            self.teams.0[swap.0].update();
        }
    }

    fn try_minimize(&self) -> Option<(usize, usize, usize, usize)> {
        let teams = &self.teams.0;

        for (t1, team) in teams.iter().enumerate() {
            for (t2, team2) in teams.iter().rev().enumerate() {
                if let Some((a, b)) = team.can_swap(&team2, &self.config, &self.players) {
                    return Some((t1, a, teams.len() - t2 - 1, b));
                }
            }
        }

        None
    }

    fn swap_steal(&mut self) {
        self.teams.update();
        let not_complete_teams = self.teams.get_not_complete();
        let not_complete_len = not_complete_teams.len();

        let mut inserted = 0;

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

                    team.add_player(candidate, &add_role);
                    team.update();
                    self.config.total_count += 1;
                    self.config.total_sr += add_role.decompose().1;
                    self.pool.0.remove(index);
                    inserted += 1;
                }
            }
        }

        if self.config.rank_limiter {
            self.config.rank_limiter = false;
            self.swap_steal();
        }

        if inserted > 0 && inserted != not_complete_len {
            self.swap_steal();
        }

        self.teams.sort(Direction::ASC);
    }

    fn increase_quality(&mut self) {
        if !self.config.rank_limiter2 {
            return;
        }

        let average = self.config.total_sr / self.config.total_count as i32;
        let cl = self.teams.clone();

        let low_supports: Vec<(usize, &Team)> = cl
            .0
            .iter()
            .enumerate()
            .filter(|(_, team)| {
                team.members_count() == 6 && team.low_support_count(self.config.limiter_max) == 2
            })
            .collect();

        let low_tanks: Vec<(usize, &Team)> =
            cl.0.iter()
                .enumerate()
                .filter(|(_, team)| {
                    team.members_count() == 6 && team.low_tank_count(self.config.limiter_max) == 2
                })
                .collect();

        let low_dps: Vec<(usize, &Team)> =
            cl.0.iter()
                .enumerate()
                .filter(|(_, team)| {
                    team.members_count() == 6 && team.low_dps_count(self.config.limiter_max) == 2
                })
                .collect();

        for (li, ls) in &low_supports {
            let res = self.t_find(SimpleRole::Support, *li, &ls, average);
            if let Some(rep) = res {
                self.teams.swap(rep.0, rep.1, rep.2, rep.3);
            }
        }

        for (li, ls) in &low_tanks {
            let res = self.t_find(SimpleRole::Tank, *li, &ls, average);
            if let Some(rep) = res {
                self.teams.swap(rep.0, rep.1, rep.2, rep.3);
            }
        }

        for (li, ls) in &low_dps {
            let res = self.t_find(SimpleRole::Dps, *li, &ls, average);
            if let Some(rep) = res {
                self.teams.swap(rep.0, rep.1, rep.2, rep.3);
            }
        }
    }

    fn t_find(
        &self,
        role: SimpleRole,
        li: usize,
        ls: &Team,
        average: i32,
    ) -> Option<(usize, usize, usize, usize)> {
        let high_supports: Vec<(usize, &Team)> = self
            .teams
            .0
            .iter()
            .enumerate()
            .filter(|(_, team)| {
                team.members_count() == 6 && team.low_support_count(self.config.limiter_max) == 0
            })
            .collect();

        for (hi, hs) in &high_supports {
            // by role
            let ls_members: Vec<(usize, &Member)> = ls
                .members
                .iter()
                .enumerate()
                .filter(|&member| member.1.role == role)
                .collect();
            for lm in ls_members {
                // by role
                let hs_members: Vec<(usize, &Member)> = hs
                    .members
                    .iter()
                    .enumerate()
                    .filter(|&member| member.1.role == role)
                    .collect();
                for hm in hs_members {
                    let new_sr_l = (ls.total_sr - lm.1.rank + hm.1.rank) / 6;
                    let new_sr_h = (hs.total_sr - hm.1.rank + lm.1.rank) / 6;
                    if (new_sr_l - average).abs() <= self.config.tolerance as i32
                        && (new_sr_h - average).abs() <= self.config.tolerance as i32
                    {
                        // return team1_id, member1_id, team2_id, member2_id
                        // perform swap
                        return Some((li, lm.0, *hi, hm.0));
                    }
                }
            }
        }

        None
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
            rank_limiter2: rank_limiter,
            dispersion_minimizer: false,
        }
    }
}

impl BalancerResult {
    fn new(teams: Teams, leftovers: PlayerPool, dispersion: i32, lows: usize) -> BalancerResult {
        BalancerResult {
            teams,
            leftovers,
            dispersion,
            anchors: lows as i32,
        }
    }
}
