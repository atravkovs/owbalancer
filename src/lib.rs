pub mod matchmaking;
pub mod players;
pub mod roles;
pub mod teams;

use matchmaking::Mathmaking;
use players::Players;
use serde::{Deserialize, Serialize};
use teams::Teams;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
) -> JsValue {
    let players: Players = player_data.into_serde().unwrap();
    console::log_2(
        &JsValue::from_str("Rank limiter"),
        &JsValue::from_bool(rank_limiter),
    );
    console::log_2(
        &JsValue::from_str("Duplicate roles"),
        &JsValue::from_bool(duplicate_roles),
    );
    let mut matchmaking = Mathmaking::new(&players, tolerance, rank_limiter, duplicate_roles);
    matchmaking.balance_players();

    JsValue::from_serde(&matchmaking.result()).unwrap()
}

#[wasm_bindgen]
pub fn balance_half(
    player_data: &JsValue,
    tolerance: u32,
    rank_limiter: bool,
    duplicate_roles: bool,
) -> JsValue {
    let players: Players = player_data.into_serde().unwrap();
    let mut matchmaking = Mathmaking::new(&players, tolerance, rank_limiter, duplicate_roles);
    matchmaking.balance_half();

    JsValue::from_serde(&matchmaking.result()).unwrap()
}

#[derive(Serialize, Deserialize)]
struct ReserveData(pub Vec<String>);

#[wasm_bindgen]
pub fn balance_final(
    player_data: &JsValue,
    tolerance: u32,
    rank_limiter: bool,
    duplicate_roles: bool,
    reserve_data: &JsValue,
    teams_data: &JsValue,
) -> JsValue {
    let players: Players = player_data.into_serde().unwrap();
    let teams: Teams = teams_data.into_serde().unwrap();
    let reserve: ReserveData = reserve_data.into_serde().unwrap();

    let mut matchmaking = Mathmaking::new(&players, tolerance, rank_limiter, duplicate_roles);
    matchmaking.add_reserve(reserve.0);
    matchmaking.add_teams(teams);
    matchmaking.balance_remaining();

    JsValue::from_serde(&matchmaking.result()).unwrap()
}
