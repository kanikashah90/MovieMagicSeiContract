use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

// Define the state of the game
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GameState {
  pub players: Vec<String>, // Store player addresses
  pub total_funds: u64,    // Total funds collected for the game
  pub active: bool,        // Flag to indicate if the game is active
  pub winner: String,      // Address of the winner
}

// Define the state of all the games
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MovieMagicContractState {
  pub games: Vec<GameState>, // Store multiple game instances
}

pub const STATE: Item<State> = Item::new("state");
