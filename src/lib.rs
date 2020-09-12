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
pub struct Team {
    pub name: String,
    pub players: Vec<Member>,
}

impl Team {
    fn new(name: String, player: Member) -> Team {
        let mut players = Vec::default();
        players.push(player);

        Team { name, players }
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
        let name = player.uuid.clone();
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
            classes.support.priority,
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
    for team in teams {
        if team.players.len() < 2 {
            team.players
                .push(Member::from_primary_player(squires.get(0).unwrap()));
            squires.remove(0);
        }
    }
}

pub fn distribue_leutenant(player: &PlayerPool, teams: &mut Vec<Team>) {
    let perfect_match = teams.iter().position(|team| {
        team.players.len() <= 2 && !team.players.get(0).unwrap().has_same_role(player)
    });

    if let Some(index) = perfect_match {
        teams
            .get_mut(index)
            .unwrap()
            .players
            .push(Member::from_primary_player(player));
        return;
    }

    for team in teams {
        if team.players.len() > 2 {
            continue;
        }

        team.players.push(Member::from_primary_player(player));
        return;
    }
}

pub fn distribute_leutenatns(teams: &mut Vec<Team>, pool: &mut Vec<PlayerPool>) {
    for i in 0..teams.len() {
        if i >= pool.len() {
            return;
        }

        let player = pool.get(i).unwrap();
        distribue_leutenant(player, teams);
    }
}

pub fn balance_players(players: &HashMap<String, Player>) -> Vec<Team> {
    let captains = get_captains(players);
    let mut preserve = preserve_uuids(&captains);
    let mut teams: Vec<Team> = init_teams(captains);

    let squires = get_squires(players, teams.len());
    preserve.extend(preserve_uuids(&squires).into_iter());
    distribute_squires(&mut teams, squires);

    let mut pool = init_pool(players, preserve);
    // From low to high sr captain
    teams.reverse();
    distribute_leutenatns(&mut teams, &mut pool);

    teams
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

#[wasm_bindgen]
pub fn balance(player_data: &JsValue) -> JsValue {
    let players: HashMap<String, Player> = player_data.into_serde().unwrap();
    let teams = balance_players(&players);

    JsValue::from_serde(&teams).unwrap()
}
