pub mod matchmaking;
pub mod players;
pub mod roles;
pub mod teams;

use matchmaking::Mathmaking;
use players::Players;
use serde::{Deserialize, Serialize};
use teams::Teams;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize, Deserialize)]
struct ReserveData(pub Vec<String>);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AsjustSr {
    pub dps: i32,
    pub tank: i32,
    pub support: i32,
    pub is_enabled: bool,
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

#[wasm_bindgen]
pub fn balance(
    player_data: &JsValue,
    tolerance: u32,
    rank_limiter: bool,
    duplicate_roles: bool,
    adjust_sr: &JsValue,
    disable_type: String,
) -> JsValue {
    let mut players: Players = player_data.into_serde().unwrap();
    let adjust: AsjustSr = adjust_sr.into_serde().unwrap();
    players.adjust_sr(adjust);

    let mut matchmaking = Mathmaking::new(&players, tolerance, rank_limiter, duplicate_roles);
    matchmaking.set_disable_type(disable_type.clone());
    matchmaking.balance_players();

    JsValue::from_serde(&matchmaking.result()).unwrap()
}

#[wasm_bindgen]
pub fn balance_half(
    player_data: &JsValue,
    tolerance: u32,
    rank_limiter: bool,
    duplicate_roles: bool,
    adjust_sr: &JsValue,
) -> JsValue {
    let mut players: Players = player_data.into_serde().unwrap();
    let adjust: AsjustSr = adjust_sr.into_serde().unwrap();
    players.adjust_sr(adjust);

    let mut matchmaking = Mathmaking::new(&players, tolerance, rank_limiter, duplicate_roles);
    matchmaking.balance_half();

    JsValue::from_serde(&matchmaking.result()).unwrap()
}

#[wasm_bindgen]
pub fn balance_final(
    player_data: &JsValue,
    tolerance: u32,
    rank_limiter: bool,
    duplicate_roles: bool,
    reserve_data: &JsValue,
    teams_data: &JsValue,
    adjust_sr: &JsValue,
) -> JsValue {
    let mut players: Players = player_data.into_serde().unwrap();
    let adjust: AsjustSr = adjust_sr.into_serde().unwrap();
    players.adjust_sr(adjust);

    let teams: Teams = teams_data.into_serde().unwrap();
    let reserve: ReserveData = reserve_data.into_serde().unwrap();

    let mut matchmaking = Mathmaking::new(&players, tolerance, rank_limiter, duplicate_roles);
    matchmaking.add_reserve(reserve.0);
    matchmaking.add_teams(teams);
    matchmaking.balance_remaining();

    JsValue::from_serde(&matchmaking.result()).unwrap()
}

#[wasm_bindgen]
pub fn boom(
    player_data: &JsValue,
    tolerance: u32,
    rank_limiter: bool,
    duplicate_roles: bool,
    reserve_data: &JsValue,
    teams_data: &JsValue,
    adjust_sr: &JsValue,
) -> JsValue {
    let mut players: Players = player_data.into_serde().unwrap();
    let adjust: AsjustSr = adjust_sr.into_serde().unwrap();
    players.adjust_sr(adjust);

    let teams: Teams = teams_data.into_serde().unwrap();
    let reserve: ReserveData = reserve_data.into_serde().unwrap();

    let mut matchmaking = Mathmaking::new(&players, tolerance, rank_limiter, duplicate_roles);
    matchmaking.add_reserve(reserve.0);
    matchmaking.add_teams(teams);
    matchmaking.boom();

    JsValue::from_serde(&matchmaking.result()).unwrap()
}
