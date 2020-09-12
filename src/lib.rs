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

#[derive(Serialize, Deserialize, Debug)]
pub enum Role {
    Tank(i32),
    Dps(i32),
    Support(i32),
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PlayerPool {
    pub uuid: String,
    pub name: String,
    pub roles: Vec<Role>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    pub name: String,
    pub players: Vec<PlayerPool>,
}

impl Team {
    fn new(name: String, player: PlayerPool) -> Team {
        let mut players = Vec::default();
        players.push(player);

        Team { name, players }
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

impl PlayerPool {
    fn new(uuid: String, name: String, roles: Vec<Role>) -> PlayerPool {
        PlayerPool { uuid, name, roles }
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

    temp_roles.sort_by(|a, b| b.priority.cmp(&a.priority));

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

pub fn get_squires(players: &HashMap<String, Player>) -> Vec<PlayerPool> {
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

    squires
}

pub fn init_pool(players: &HashMap<String, Player>) -> Vec<PlayerPool> {
    let mut pool = Vec::default();

    for (_, player) in players {
        pool.push(PlayerPool::new(
            player.identity.uuid.clone(),
            player.identity.name.clone(),
            get_roles(player),
        ));
    }

    pool.sort_by(|a, b| get_priority_rank(&b.roles).cmp(&get_priority_rank(&a.roles)));

    pool
}

pub fn init_teams(captains: Vec<PlayerPool>) -> Vec<Team> {
    let mut teams = Vec::default();

    for cap in captains {
        teams.push(Team::new(cap.name.clone(), cap));
    }

    teams
}

pub fn balance(players: &HashMap<String, Player>) -> Vec<Team> {
    let captains = get_captains(players);
    let mut teams: Vec<Team> = init_teams(captains);

    let squires = get_squires(players);

    teams
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console::log_1(&JsValue::from_str("Hello world!!"));

    Ok(())
}

#[wasm_bindgen]
pub fn greet(player_data: &JsValue) -> JsValue {
    let players: HashMap<String, Player> = player_data.into_serde().unwrap();
    let teams = balance(&players);

    JsValue::from_serde(&teams).unwrap()
}
