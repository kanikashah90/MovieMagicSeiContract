use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AdventureState {
    pub votes: HashMap<String, u32>,
}

// Define the state of the game
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GameState {
    pub name: String,         // Name of the game
    pub players: Vec<String>, // Store player addresses
    pub total_funds: u64,     // Total funds collected for the game
    pub adventure_funds: u64, // Total funds to be distributed for the adventures
    pub initiated: bool,      // Flag to indicate if the game has been initiated
    pub started: bool,        // Flag to indicate if the game has been started
    pub ended: bool,          // Flag to indicate if the game has been ended
    pub winner: String,       // Address of the winner
    pub winning_reward: u64,  // Winning reward
    pub adventure_votes: Vec<HashMap<String, u32>>,
    pub adventure_winners: Vec<Vec<String>>,
    pub adventure_rewards: Vec<u64>,
    pub adventure_winning_votes: Vec<u32>,
    pub num_of_adventures: u32, // Number of adventures
}

// Define the state of all the games
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MovieMagicContractState {
    pub games: Vec<GameState>, // Store multiple game instances
    pub owner: Addr,           // Address of the contract owner
}

#[derive(Serialize, Deserialize)]
pub struct QueryResp {
    pub message: String,
}

// pub const STATE: Item<State> = Item::new("state");
pub const STATE: Item<MovieMagicContractState> = Item::new("state");
