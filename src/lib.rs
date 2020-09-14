pub mod matchmaking;
pub mod players;
pub mod roles;
pub mod teams;

use matchmaking::Mathmaking;
use players::Players;
use wasm_bindgen::prelude::*;

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
pub fn balance(player_data: &JsValue, tolerance: u32) -> JsValue {
    let players: Players = player_data.into_serde().unwrap();
    let mut matchmaking = Mathmaking::new(&players, tolerance);
    matchmaking.balance_players();

    JsValue::from_serde(&matchmaking.result()).unwrap()
}
