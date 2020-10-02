use crate::matchmaking::Config;
use crate::players::{Candidate, Direction, PlayerPool, Players};
use crate::roles::{Role, RolesFilter, SimpleRole};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Member {
    pub rank: i32,
    pub uuid: String,
    pub name: String,
    pub primary: bool,
    pub secondary: bool,
    pub role: SimpleRole,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub uuid: Uuid,
    pub avg_sr: f32,
    pub name: String,
    pub total_sr: i32,
    pub members: Vec<Member>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Teams(pub Vec<Team>);

impl Member {
    pub fn new(uuid: String, name: String, role: Role) -> Member {
        let (simple, rank) = role.decompose();

        Member {
            uuid,
            name,
            rank,
            role: simple,
            primary: role.is_primary(),
            secondary: role.is_secondary(),
        }
    }

    pub fn from_primary_player(candidate: &Candidate) -> Member {
        let uuid = candidate.uuid.clone();
        let name = candidate.name.clone();
        let role = candidate.roles.get_primary().clone();

        return Member::new(uuid, name, role);
    }

    pub fn has_same_role(&self, player: &Candidate) -> bool {
        let primary_role = player.roles.get_primary();

        primary_role.is_same(&self.role)
    }

    fn is_dps(&self) -> bool {
        self.role == SimpleRole::Dps
    }
}

impl Team {
    pub fn new(name: String, player: Member) -> Team {
        let mut members = Vec::default();
        members.push(player);
        let uuid = Uuid::new_v4();

        Team {
            name,
            uuid,
            members,
            avg_sr: 0.0,
            total_sr: 0,
        }
    }

    pub fn get_captain(&self) -> &Member {
        self.members.first().unwrap()
    }

    pub fn get_squire(&self) -> &Member {
        self.members.get(1).unwrap()
    }

    pub fn get_leutenant(&self) -> &Member {
        self.members.get(2).unwrap()
    }

    pub fn add_primary_player(&mut self, candidate: &Candidate) {
        self.members.push(Member::from_primary_player(candidate));
    }

    pub fn add_player(&mut self, candidate: &Candidate, target_role: &Role) {
        self.members.push(Member::new(
            candidate.uuid.clone(),
            candidate.name.clone(),
            target_role.clone(),
        ));
    }

    pub fn dps_count(&self) -> usize {
        self.count_role(&SimpleRole::Dps)
    }

    pub fn low_role_count(&self, role: &SimpleRole, threshold: i32) -> usize {
        match role {
            SimpleRole::Dps => self.low_dps_count(threshold),
            SimpleRole::Tank => self.low_tank_count(threshold),
            SimpleRole::Support => self.low_support_count(threshold),
        }
    }

    pub fn total_low_role_count(&self, threshold: i32) -> usize {
        let dps = if self.low_dps_count(threshold) > 1 {
            1
        } else {
            0
        };
        let support = if self.low_support_count(threshold) > 1 {
            1
        } else {
            0
        };
        let tank = if self.low_tank_count(threshold) > 1 {
            1
        } else {
            0
        };

        dps + support + tank
    }

    pub fn low_dps_count(&self, threshold: i32) -> usize {
        self.count_role_lows(&SimpleRole::Dps, threshold)
    }

    pub fn has_dps_duplicate(&self, primary: bool, secondary: bool) -> bool {
        self.find_duplicates(&SimpleRole::Dps, primary, secondary)
    }

    pub fn support_count(&self) -> usize {
        self.count_role(&SimpleRole::Support)
    }

    pub fn has_support_duplicate(&self, primary: bool, secondary: bool) -> bool {
        self.find_duplicates(&SimpleRole::Support, primary, secondary)
    }

    pub fn low_support_count(&self, threshold: i32) -> usize {
        self.count_role_lows(&SimpleRole::Support, threshold)
    }

    pub fn tank_count(&self) -> usize {
        self.count_role(&SimpleRole::Tank)
    }

    pub fn has_tank_duplicate(&self, primary: bool, secondary: bool) -> bool {
        self.find_duplicates(&SimpleRole::Tank, primary, secondary)
    }

    pub fn low_tank_count(&self, threshold: i32) -> usize {
        self.count_role_lows(&SimpleRole::Tank, threshold)
    }

    pub fn update(&mut self) -> (f32, i32) {
        let total_sr = self.total_sr();
        let avg_sr = total_sr as f32 / self.members_count() as f32;

        self.avg_sr = avg_sr;
        self.total_sr = total_sr;

        (avg_sr, total_sr)
    }

    pub fn members_count(&self) -> usize {
        self.members.len()
    }

    pub fn get_missing_role(&self) -> Option<SimpleRole> {
        if self.support_count() < 2 {
            return Some(SimpleRole::Support);
        }

        if self.dps_count() < 2 {
            return Some(SimpleRole::Dps);
        }

        if self.tank_count() < 2 {
            return Some(SimpleRole::Tank);
        }

        None
    }

    pub fn get_range(&self, config: &Config) -> (i32, i32) {
        let players_count = 6;
        let tolerance_range = config.tolerance * players_count;

        let target_sr = if self.members_count() as u32 != players_count {
            ((config.players_average * players_count as i32) - self.total_sr)
                / (players_count as i32 - self.members_count() as i32)
        } else {
            self.avg_sr as i32
        };

        let min_sr = target_sr - tolerance_range as i32;
        let max_sr = target_sr + tolerance_range as i32;

        (min_sr, max_sr)
    }

    pub fn try_replace(
        &self,
        candidate: &Candidate,
        target_role: &SimpleRole,
        range: (i32, i32),
        db: &PlayerPool,
        target_team: &Team,
        config: &Config,
    ) -> Option<usize> {
        for role in &candidate.roles.0 {
            let (simple, _) = role.decompose();
            let position = self.members.iter().position(|member| {
                let is_self = target_team.uuid == self.uuid;

                if member.role != simple.clone() {
                    return false;
                }

                if !is_self && !self.pfsr(candidate, member, config) {
                    return false;
                }

                if let Some(player) = db.get_by_id(member.uuid.clone()) {
                    if let Some(player_role) = player.roles.get_by_simple(&target_role) {
                        if is_self && !self.pfsr2(candidate, member, config, player_role) {
                            return false;
                        }

                        return player_role.is_in_range(range)
                            && player_role.fits_team_limit(target_team, config);
                    }
                }

                false
            });

            if position.is_some() {
                return position;
            }
        }

        None
    }

    pub fn pfsr(&self, candidate: &Candidate, member: &Member, config: &Config) -> bool {
        if let Some(role) = candidate.roles.get_by_simple(&member.role) {
            let rank = role.decompose().1;
            let new_average = (config.total_sr + rank) as f32 / (config.total_count + 1) as f32;
            let team_size = self.members_count();
            let new_sr = (self.total_sr + rank - member.rank) as f32 / team_size as f32;

            ((new_sr - new_average).abs().floor() as u32) <= config.tolerance
                && role.fits_team_limit(&self, config)
        } else {
            false
        }
    }

    pub fn pfsr2(
        &self,
        candidate: &Candidate,
        member: &Member,
        config: &Config,
        player_role: &Role,
    ) -> bool {
        if let Some(role) = candidate.roles.get_by_simple(&member.role) {
            let rank = role.decompose().1;
            let rank2 = player_role.decompose().1;
            let new_average = (config.total_sr - member.rank + rank2 + rank) as f32
                / (config.total_count + 1) as f32;
            let team_size = self.members_count();
            let new_sr = (self.total_sr + rank + rank2 - member.rank) as f32 / team_size as f32;

            ((new_sr - new_average).abs().floor() as u32) <= config.tolerance
                && role.fits_team_limit(&self, config)
        } else {
            false
        }
    }

    pub fn fits_sr(&self, player_sr: i32, new_average: f32, tolerance: u32) -> bool {
        let team_size = self.members_count();
        let new_sr = (self.total_sr + player_sr) as f32 / (team_size + 1) as f32;
        ((new_sr - new_average).abs().floor() as u32) <= tolerance * (6 - team_size as u32)
    }

    pub fn can_swap(
        &self,
        team: &Team,
        config: &Config,
        players: &Players,
    ) -> Option<(usize, usize)> {
        let avg = config.total_sr / config.total_count as i32;

        for (i, mem) in self.members.iter().enumerate() {
            let p1_base = &players.0.get(mem.uuid.as_str()).unwrap();
            let p1 = &p1_base.identity;
            if p1.is_captain || p1.is_squire {
                continue;
            }

            for (j, mem2) in team.members.iter().enumerate() {
                let p2_base = &players.0.get(mem2.uuid.as_str()).unwrap();
                let p2 = &p2_base.identity;
                if p2.is_captain || p2.is_squire {
                    continue;
                }

                if mem.role == mem2.role {
                    if config.rank_limiter2 {
                        if (mem.rank < 2500 && team.low_role_count(&mem.role, 2500) == 1)
                            || (mem2.rank < 2500 && self.low_role_count(&mem2.role, 2500) == 1)
                        {
                            continue;
                        }
                    }

                    if config.duplicate_roles2 {
                        let mem1_class = p1_base.stats.classes.get_class(&mem.role);
                        let mem2_class = p2_base.stats.classes.get_class(&mem2.role);

                        if mem1_class.primary == mem2_class.primary
                            || mem1_class.secondary == mem2_class.secondary
                        {
                            continue;
                        }
                    }

                    let newsr =
                        (self.total_sr - mem.rank + mem2.rank) / self.members_count() as i32;
                    let newsr2 =
                        (team.total_sr - mem2.rank + mem.rank) / team.members_count() as i32;
                    let disp1 = (self.avg_sr as i32 - avg).abs();
                    let disp2 = (team.avg_sr as i32 - avg).abs();
                    let newdisp1 = (newsr - avg).abs();
                    let newdisp2 = (newsr2 - avg).abs();

                    if newdisp1 < disp1 && newdisp2 < disp2 {
                        return Some((i, j));
                    }
                }
            }
        }

        None
    }

    fn total_sr(&self) -> i32 {
        self.members.iter().map(|member| member.rank).sum()
    }

    fn count_role(&self, role: &SimpleRole) -> usize {
        self.members
            .iter()
            .filter(|&member| member.role == *role)
            .count()
    }

    fn count_role_lows(&self, role: &SimpleRole, threshold: i32) -> usize {
        self.members
            .iter()
            .filter(|&member| member.role == *role && member.rank < threshold)
            .count()
    }

    fn find_duplicates(&self, role: &SimpleRole, primary: bool, secondary: bool) -> bool {
        if primary {
            if self
                .members
                .iter()
                .find(|&member| member.role == *role && member.primary)
                .is_some()
            {
                return true;
            }
        }

        if secondary {
            if self
                .members
                .iter()
                .find(|&member| member.role == *role && member.secondary)
                .is_some()
            {
                return true;
            }
        }

        false
    }
}

impl Teams {
    pub fn reverse(&mut self) {
        self.0.reverse();
    }

    pub fn update(&mut self) {
        for team in &mut self.0 {
            team.update();
        }
    }

    pub fn get_mut(&mut self, index: usize) -> &mut Team {
        self.0.get_mut(index).unwrap()
    }

    pub fn get(&self, index: usize) -> &Team {
        self.0.get(index).unwrap()
    }

    pub fn total_low_role_count(&self, threshold: i32) -> usize {
        self.0
            .iter()
            .map(|team| team.total_low_role_count(threshold))
            .sum()
    }

    pub fn get_stats(&self) -> (i32, usize) {
        let mut total_sr = 0;
        let mut total_count = 0;

        for team in &self.0 {
            total_sr += team.total_sr;
            total_count += team.members_count();
        }

        (total_sr, total_count)
    }

    pub fn swap(&mut self, team1: usize, member1: usize, team2: usize, member2: usize) {
        let memb1 = self.get(team1).members.get(member1).unwrap().clone();
        let memb2 = self.get(team2).members.get(member2).unwrap().clone();
        self.get_mut(team1).members.remove(member1);
        self.get_mut(team2).members.remove(member2);
        self.get_mut(team1).members.push(memb2);
        self.get_mut(team2).members.push(memb1);
        self.get_mut(team1).update();
        self.get_mut(team2).update();
    }

    pub fn get_not_complete(&self) -> Vec<usize> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(id, team)| {
                if team.members_count() < 6 {
                    Some(id)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn replace_leftover(
        &self,
        leftover: &Candidate,
        role: &SimpleRole,
        range: (i32, i32),
        db: &PlayerPool,
        target_team: &Team,
        config: &Config,
    ) -> Option<(usize, usize)> {
        for (team_index, team) in self.0.iter().enumerate() {
            if let Some(replacement) =
                team.try_replace(leftover, role, range, db, target_team, config)
            {
                return Some((team_index, replacement));
            }
        }

        None
    }

    pub fn sort(&mut self, direction: Direction) {
        self.0.sort_by(|a, b| {
            if let Some(ordering) = a.avg_sr.partial_cmp(&b.avg_sr) {
                if direction == Direction::DESC {
                    return ordering.reverse();
                }

                ordering
            } else {
                Ordering::Equal
            }
        })
    }

    pub fn filter_dps_captains(&mut self) -> Vec<&mut Team> {
        self.0
            .iter_mut()
            .filter(|team| team.get_captain().is_dps())
            .collect()
    }

    pub fn distribute_squires_to_dps(&mut self, squires: &mut PlayerPool) {
        let roles_filter = RolesFilter(vec![SimpleRole::Tank, SimpleRole::Dps]);

        let dps_teams = self.filter_dps_captains();
        let mut worthy_squires = squires.filter_by_roles(roles_filter);
        let mut cache_squires = Vec::default();

        for team in dps_teams {
            if worthy_squires.is_empty() {
                break;
            }

            let candidate = worthy_squires.pop().unwrap();
            cache_squires.push(candidate);
            team.add_primary_player(squires.0.get(candidate).unwrap());
        }

        squires.dispose_of(cache_squires);
    }

    pub fn distribute_squires(&mut self, squires: &mut PlayerPool) {
        for team in &mut self.0 {
            if team.members_count() < 2 {
                if let Some(candidate) = squires.0.pop() {
                    team.add_primary_player(&candidate);
                }
            }
        }
    }

    pub fn distribute_leutenants(&mut self, pool: &mut PlayerPool, config: &Config) {
        let mut offset = 0;
        for _ in 0..self.teams_count() {
            offset = pool.distribute_leutenant(self, offset, config);
        }
    }

    pub fn distribute_fillers(&mut self, pool: &mut PlayerPool, config: &Config) {
        for team in &mut self.0 {
            pool.distribute_filler(team, config);
        }
    }

    pub fn distribute_ensigns(&mut self, pool: &mut PlayerPool, config: &Config) {
        let mut offset = 0;
        for _ in 0..self.teams_count() {
            offset = pool.distribute_ensign(self, offset, config);
        }
    }

    pub fn find_mate(
        &mut self,
        candidate: &Candidate,
        max_member_count: usize,
        config: &Config,
    ) -> Option<&mut Team> {
        self.0.iter_mut().find(|team| {
            team.members_count() <= max_member_count
                && candidate.get_primary_role().fits_team(team, config)
                && !team.get_captain().has_same_role(candidate)
        })
    }

    pub fn find_perfect_ensign(
        &mut self,
        candidate: &Candidate,
        config: &Config,
    ) -> Option<&mut Team> {
        self.0.iter_mut().find(|team| {
            team.members_count() <= 3
                && candidate.get_primary_role().fits_team(team, config)
                && !team.get_captain().has_same_role(candidate)
                && !team.get_leutenant().has_same_role(candidate)
        })
    }

    pub fn find_team(
        &mut self,
        max_size: usize,
        target_role: &Role,
        config: &Config,
    ) -> Option<&mut Team> {
        self.0
            .iter_mut()
            .find(|team| team.members_count() <= max_size && target_role.fits_team(team, config))
    }

    pub fn fit_player(
        &mut self,
        player_sr: i32,
        new_average: f32,
        config: &Config,
        target_role: &Role,
    ) -> Option<&mut Team> {
        let mut teams: Vec<usize> = self
            .0
            .iter()
            .enumerate()
            .filter_map(|(index, team)| {
                let team_size = team.members_count();

                if (team_size + 1) <= 6
                    && target_role.fits_team(team, config)
                    && team.fits_sr(player_sr, new_average, config.tolerance)
                {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();

        teams.sort_by(|&a, &b| {
            let team1 = &self.0[a];
            let team2 = &self.0[b];
            let c1 = team1.members_count();
            let c2 = team2.members_count();

            c1.cmp(&c2)
        });

        if let Some(&a) = teams.get(0) {
            self.0.get_mut(a)
        } else {
            None
        }
    }

    fn teams_count(&self) -> usize {
        self.0.len()
    }
}

impl From<PlayerPool> for Teams {
    fn from(player_pool: PlayerPool) -> Self {
        let teams = player_pool
            .0
            .into_iter()
            .map(|player| Team::new(player.name.clone(), Member::from_primary_player(&player)))
            .collect();

        Teams(teams)
    }
}
