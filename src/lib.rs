use rand::rngs::OsRng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub uuid: String,
    pub name: String,
    pub is_squire: bool,
    pub is_captain: bool,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Role {
    Tank(i32),
    Dps(i32),
    Support(i32),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SimpleRole {
    Tank,
    Dps,
    Support,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PlayerPool {
    pub uuid: String,
    pub name: String,
    pub roles: Vec<Role>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Member {
    pub rank: i32,
    pub uuid: String,
    pub name: String,
    pub role: SimpleRole,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub avg_sr: f32,
    pub name: String,
    pub total_sr: i32,
    pub players: Vec<Member>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BalancerResult {
    pub teams: Vec<Team>,
    pub leftovers: Vec<PlayerPool>,
}

impl BalancerResult {
    fn new(from: (Vec<Team>, Vec<PlayerPool>)) -> BalancerResult {
        BalancerResult {
            teams: from.0,
            leftovers: from.1,
        }
    }
}

impl Team {
    fn new(name: String, player: Member) -> Team {
        let mut players = Vec::default();
        players.push(player);

        Team {
            name,
            players,
            avg_sr: 0.0,
            total_sr: 0,
        }
    }

    fn fits_role(&self, role: &Role) -> bool {
        let dps_count = self
            .players
            .iter()
            .filter(|&member| member.role == SimpleRole::Dps)
            .count();
        let support_count = self
            .players
            .iter()
            .filter(|&member| member.role == SimpleRole::Support)
            .count();
        let tank_count = self
            .players
            .iter()
            .filter(|&member| member.role == SimpleRole::Tank)
            .count();

        match role {
            Role::Dps(_) => (dps_count + 1) <= 2,
            Role::Support(_) => (support_count + 1) <= 2,
            Role::Tank(_) => (tank_count + 1) <= 2,
        }
    }
}

impl Member {
    fn new(uuid: String, name: String, role: Role) -> Member {
        let (role, rank) = role.decompose();

        Member {
            uuid,
            name,
            role,
            rank,
        }
    }

    fn from_primary_player(player: &PlayerPool) -> Member {
        let uuid = player.uuid.clone();
        let name = player.name.clone();
        let role = player.roles.get(0).unwrap().clone();

        return Member::new(uuid, name, role);
    }

    fn has_same_role(&self, player: &PlayerPool) -> bool {
        let p_role = player.roles.get(0).unwrap();

        match p_role {
            Role::Dps(_) => SimpleRole::Dps == self.role,
            Role::Support(_) => SimpleRole::Support == self.role,
            Role::Tank(_) => SimpleRole::Tank == self.role,
        }
    }
}

pub struct RolePriority {
    pub role: Role,
    pub priority: i16,
}

impl RolePriority {
    fn new(role: Role, priority: i16) -> RolePriority {
        RolePriority { role, priority }
    }
}

impl Role {
    fn is_same(&self, role: &str) -> bool {
        match self {
            Role::Dps(_) => role == "dps",
            Role::Support(_) => role == "support",
            Role::Tank(_) => role == "tank",
        }
    }

    fn decompose(&self) -> (SimpleRole, i32) {
        match self {
            Role::Dps(rank) => (SimpleRole::Dps, *rank),
            Role::Support(rank) => (SimpleRole::Support, *rank),
            Role::Tank(rank) => (SimpleRole::Tank, *rank),
        }
    }
}

impl PlayerPool {
    fn new(uuid: String, name: String, roles: Vec<Role>) -> PlayerPool {
        PlayerPool { uuid, name, roles }
    }

    fn has_specified_primary_role(&self, roles: &Vec<&str>) -> bool {
        let primary_role = self.roles.first().unwrap();

        for role in roles.clone() {
            if primary_role.is_same(role) {
                return true;
            }
        }

        false
    }
}

pub fn get_roles(player: &Player) -> Vec<Role> {
    let classes = &player.stats.classes;
    let mut temp_roles: Vec<RolePriority> = Vec::new();

    if classes.dps.is_active {
        temp_roles.push(RolePriority::new(
            Role::Dps(classes.dps.rank),
            classes.dps.priority,
        ));
    }

    if classes.support.is_active {
        temp_roles.push(RolePriority::new(
            Role::Support(classes.support.rank),
            classes.support.priority,
        ));
    }

    if classes.tank.is_active {
        temp_roles.push(RolePriority::new(
            Role::Tank(classes.tank.rank),
            classes.tank.priority,
        ));
    }

    temp_roles.sort_by(|a, b| a.priority.cmp(&b.priority));

    let mut roles = Vec::default();

    for role in temp_roles {
        roles.push(role.role);
    }

    roles
}

pub fn get_priority_rank(roles: &Vec<Role>) -> i32 {
    let priority_role = &roles[0];

    match priority_role {
        Role::Dps(rank) => *rank,
        Role::Support(rank) => *rank,
        Role::Tank(rank) => *rank,
    }
}

pub fn get_captains(players: &HashMap<String, Player>) -> Vec<PlayerPool> {
    let mut captains = Vec::default();
    for (uuid, player) in players {
        if player.identity.is_captain {
            captains.push(PlayerPool::new(
                uuid.clone(),
                player.identity.name.clone(),
                get_roles(player),
            ));
        }
    }

    captains.sort_by(|a, b| get_priority_rank(&b.roles).cmp(&get_priority_rank(&a.roles)));

    captains
}

pub fn get_squires(players: &HashMap<String, Player>, cap_count: usize) -> Vec<PlayerPool> {
    let mut squires = Vec::default();

    for (uuid, player) in players {
        if player.identity.is_squire {
            squires.push(PlayerPool::new(
                uuid.clone(),
                player.identity.name.clone(),
                get_roles(player),
            ));
        }
    }

    squires.sort_by(|a, b| get_priority_rank(&a.roles).cmp(&get_priority_rank(&b.roles)));

    while squires.len() > cap_count {
        squires.pop();
    }

    squires
}

pub fn init_pool(players: &HashMap<String, Player>, exclude: Vec<String>) -> Vec<PlayerPool> {
    let mut pool = Vec::default();

    for (_, player) in players {
        if !exclude.contains(&player.identity.uuid) {
            pool.push(PlayerPool::new(
                player.identity.uuid.clone(),
                player.identity.name.clone(),
                get_roles(player),
            ));
        }
    }

    pool.sort_by(|a, b| get_priority_rank(&b.roles).cmp(&get_priority_rank(&a.roles)));

    pool
}

pub fn init_teams(captains: Vec<PlayerPool>) -> Vec<Team> {
    let mut teams = Vec::default();

    for cap in captains {
        teams.push(Team::new(
            cap.name.clone(),
            Member::from_primary_player(&cap),
        ));
    }

    console::log_1(&JsValue::from_serde(&teams).unwrap());

    teams
}

pub fn preserve_uuids(player_pool: &Vec<PlayerPool>) -> Vec<String> {
    let mut preserved = Vec::default();

    for player in player_pool {
        preserved.push(player.uuid.clone());
    }

    preserved
}

pub fn locate_player(players: &Vec<PlayerPool>, roles: &Vec<&str>) -> Option<usize> {
    players
        .iter()
        .position(|player| player.has_specified_primary_role(roles))
}

pub fn distribute_squires_to_dps(teams: &mut Vec<Team>, squires: &mut Vec<PlayerPool>) {
    let roles = vec!["tank", "dps"];

    for team in teams {
        if let SimpleRole::Dps = team.players.first().unwrap().role {
            // Find Tank / DD squire
            let located = locate_player(squires, &roles);
            if let Some(position) = located {
                let squire = squires.get(position).unwrap();
                team.players.push(Member::from_primary_player(squire));
                squires.remove(position);
            }
        }
    }
}

pub fn distribute_squires(teams: &mut Vec<Team>, squires: Vec<PlayerPool>) {
    let mut squires = squires;
    // Deal with DPS captains first
    distribute_squires_to_dps(teams, &mut squires);

    // Distribute other players
    let mut i = 0;
    for team in teams {
        if team.players.len() < 2 {
            team.players
                .push(Member::from_primary_player(squires.get(i).unwrap()));
            i += 1;
        }
    }

    squires.drain(0..i);
}

pub fn distribute_leutenant(player: &PlayerPool, teams: &mut Vec<Team>) -> bool {
    let perfect_match = teams.iter().position(|team| {
        team.players.len() <= 2 && !team.players.get(0).unwrap().has_same_role(player)
    });

    if let Some(index) = perfect_match {
        teams
            .get_mut(index)
            .unwrap()
            .players
            .push(Member::from_primary_player(player));
        return true;
    }

    for team in teams {
        if team.players.len() > 2 {
            continue;
        }

        if team.fits_role(player.roles.get(0).unwrap()) {
            team.players.push(Member::from_primary_player(player));
            return true;
        }
    }

    false
}

pub fn distribute_ensign(player: &PlayerPool, teams: &mut Vec<Team>) -> bool {
    let target_role = player.roles.get(0).unwrap();

    let perfect_match = teams.iter().position(|team| {
        let cap = team.players.get(0).unwrap();
        let leut = team.players.get(2).unwrap();

        !cap.has_same_role(player) && !leut.has_same_role(player) && team.players.len() <= 3
    });

    if let Some(index) = perfect_match {
        teams
            .get_mut(index)
            .unwrap()
            .players
            .push(Member::from_primary_player(player));
        return true;
    }

    let perfect_match = teams.iter().position(|team| {
        let cap = team.players.get(0).unwrap();

        !cap.has_same_role(player) && team.fits_role(target_role) && team.players.len() <= 3
    });

    if let Some(index) = perfect_match {
        teams
            .get_mut(index)
            .unwrap()
            .players
            .push(Member::from_primary_player(player));
        return true;
    }

    let perfect_match = teams
        .iter()
        .position(|team| team.players.len() <= 3 && team.fits_role(target_role));
    if let Some(index) = perfect_match {
        teams
            .get_mut(index)
            .unwrap()
            .players
            .push(Member::from_primary_player(player));
        return true;
    }

    false
}

pub fn distribute_leutenatns(teams: &mut Vec<Team>, pool: &mut Vec<PlayerPool>) {
    let mut i = 0;
    let mut count = 0;
    while count < teams.len() {
        if pool.len() <= 0 {
            return;
        }

        let player = pool.get(i).unwrap();
        if distribute_leutenant(player, teams) {
            pool.remove(i);
            count += 1;
        } else {
            i += 1;
        }
    }
}

pub fn distribute_ensigns(teams: &mut Vec<Team>, pool: &mut Vec<PlayerPool>) {
    let mut i = 0;
    let mut count = 0;
    while count < teams.len() {
        if pool.len() <= 0 {
            return;
        }

        let player = pool.get(i).unwrap();
        if distribute_ensign(player, teams) {
            pool.remove(i);
            count += 1;
        } else {
            i += 1;
        }
    }
}

pub fn calculate_every_team_average(teams: &mut Vec<Team>) {
    for team in teams {
        let mut total_sr: i32 = 0;
        for player in &team.players {
            total_sr += player.rank;
        }

        let avg_sr = total_sr as f32 / team.players.len() as f32;

        team.avg_sr = avg_sr;
        team.total_sr = total_sr;
    }
}

pub fn sort_by_average(teams: &mut Vec<Team>) {
    teams.sort_by(|a, b| a.avg_sr.partial_cmp(&b.avg_sr).unwrap());
}

// (average, total_sr, total_count)
pub fn get_total_average(teams: &mut Vec<Team>) -> (f32, i32, usize) {
    calculate_every_team_average(teams);
    let mut total_sr = 0;
    let mut total_count = 0;

    for team in teams {
        total_sr += team.total_sr;
        total_count += team.players.len();
    }

    let average = total_sr as f32 / total_count as f32;

    return (average, total_sr, total_count);
}

// (role_index, total_sr, total_players, tolerance)
// Option<playerSr>
pub fn fit_player(
    teams: &mut Vec<Team>,
    player: &PlayerPool,
    data: (bool, i32, usize, u32),
) -> Option<i32> {
    let (sec_roles, total_sr, total_players, tolerance) = data;
    let parse_roles_count = if !sec_roles { 1 } else { player.roles.len() };

    for role_index in 0..parse_roles_count {
        let target_role = player.roles.get(role_index);

        if target_role.is_none() {
            return None;
        }

        let target_role = target_role.unwrap();
        let player_sr = target_role.decompose().1;
        let new_average = (total_sr + player_sr) as f32 / (total_players + 1) as f32;

        let perfect_match = teams.iter().position(|team| {
            let team_size = team.players.len();
            let new_sr = (team.total_sr + player_sr) as f32 / (team_size + 1) as f32;

            (team_size + 1) <= 6
                && team.fits_role(target_role)
                && ((new_sr - new_average).abs().floor() as u32) <= tolerance
        });

        if let Some(index) = perfect_match {
            let team = teams.get_mut(index).unwrap();

            team.total_sr += player_sr;
            team.avg_sr = team.total_sr as f32 / (team.players.len() + 1) as f32;
            team.players.push(Member::new(
                player.uuid.clone(),
                player.name.clone(),
                target_role.clone(),
            ));

            return Some(player_sr);
        }
    }

    None
}

pub fn sort_remaining(
    teams: &mut Vec<Team>,
    pool: &mut Vec<PlayerPool>,
    tolerance: u32,
    total_sr: i32,
    total_count: usize,
    delta: u32,
    sec_roles: bool,
) -> Vec<PlayerPool> {
    let mut leftovers: Vec<PlayerPool> = Vec::default();
    let mut total_sr = total_sr;
    let mut total_count = total_count;
    let mut sec_roles = sec_roles;
    let mut added_players = 0;

    pool.shuffle(&mut OsRng::default());

    for player in pool {
        if let Some(sr) = fit_player(teams, player, (sec_roles, total_sr, total_count, tolerance)) {
            total_sr += sr;
            total_count += 1;
            added_players += 1;
        } else {
            leftovers.push(player.clone());
        }
    }

    if added_players == 0 && delta == 0 && !sec_roles && leftovers.len() > 0 {
        sec_roles = true;
        return sort_remaining(
            teams,
            &mut leftovers,
            tolerance,
            total_sr,
            total_count,
            delta,
            sec_roles,
        );
    } else if leftovers.len() > 0 && (added_players > 0 || !sec_roles) {
        return sort_remaining(
            teams,
            &mut leftovers,
            tolerance,
            total_sr,
            total_count,
            added_players,
            sec_roles,
        );
    }

    return leftovers;
}

pub fn balance_players(
    players: &HashMap<String, Player>,
    tolerance: u32,
) -> (Vec<Team>, Vec<PlayerPool>) {
    let captains = get_captains(players);
    let mut preserve = preserve_uuids(&captains);
    console::log_2(
        &JsValue::from_str("Initial captains: "),
        &JsValue::from_serde(&captains).unwrap(),
    );
    let mut teams: Vec<Team> = init_teams(captains);

    let squires = get_squires(players, teams.len());
    preserve.extend(preserve_uuids(&squires).into_iter());
    distribute_squires(&mut teams, squires);

    let mut pool = init_pool(players, preserve);
    // From low to high sr captain
    teams.reverse();
    distribute_leutenatns(&mut teams, &mut pool);

    calculate_every_team_average(&mut teams);
    sort_by_average(&mut teams);
    distribute_ensigns(&mut teams, &mut pool);

    let (_, total_sr, total_count) = get_total_average(&mut teams);
    let delta = 1;
    let sec_roles = false;
    let leftovers = sort_remaining(
        &mut teams,
        &mut pool,
        tolerance,
        total_sr,
        total_count,
        delta,
        sec_roles,
    );

    (teams, leftovers)
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

#[wasm_bindgen]
pub fn balance(player_data: &JsValue, tolerance: u32) -> JsValue {
    let players: HashMap<String, Player> = player_data.into_serde().unwrap();
    let output = balance_players(&players, tolerance);

    let result = BalancerResult::new(output);

    JsValue::from_serde(&result).unwrap()
}
