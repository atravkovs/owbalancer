pub mod matchmaking;
pub mod players;
pub mod roles;
pub mod teams;

use matchmaking::{BalancerResult, Mathmaking};
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

#[wasm_bindgen(module = "/js/logger.js")]
extern "C" {
    pub fn wasm_log(message: String);
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

pub fn run_matchmaking(
    player_data: &JsValue,
    tolerance: u32,
    rank_limiter: bool,
    duplicate_roles: bool,
    adjust_sr: &JsValue,
    disable_type: String,
    dispersion_minimizer: bool,
) -> BalancerResult {
    let mut players: Players = player_data.into_serde().unwrap();
    let adjust: AsjustSr = adjust_sr.into_serde().unwrap();
    players.adjust_sr(adjust);

    let mut matchmaking = Mathmaking::new(&players, tolerance, rank_limiter, duplicate_roles);
    matchmaking.set_disable_type(disable_type.clone());

    if dispersion_minimizer {
        matchmaking.enable_dispersion_minimizer();
    }

    matchmaking.balance_players();

    matchmaking.result()
}

#[wasm_bindgen]
pub fn balance(
    player_data: &JsValue,
    tolerance: u32,
    rank_limiter: bool,
    duplicate_roles: bool,
    adjust_sr: &JsValue,
    disable_type: String,
    dispersion_minimizer: bool,
    tries_count: u32,
) -> JsValue {
    let mut results = Vec::default();

    for _ in 0..tries_count {
        let result = run_matchmaking(
            player_data,
            tolerance,
            rank_limiter,
            duplicate_roles,
            adjust_sr,
            disable_type.clone(),
            dispersion_minimizer,
        );
        results.push(result);
    }

    JsValue::from_serde(&results).unwrap()
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

    let mut results = Vec::default();
    results.push(matchmaking.result());

    JsValue::from_serde(&results).unwrap()
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

    let mut results = Vec::default();
    results.push(matchmaking.result());

    JsValue::from_serde(&results).unwrap()
}
