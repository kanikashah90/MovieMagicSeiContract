use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
}

// #[cw_serde]
// pub enum ExecuteMsg {
//     Increment {},
//     Reset { count: i32 },
// }

#[cw_serde]
pub struct GamePlayerVote {
    pub player: String,
    pub vote: u32,
}

#[cw_serde]
pub enum ExecuteMsg {
    InitGame {
        name: String,
        player: String,
        game_stake: u64,
        num_of_adventures: u32,
    },
    AddGamePlayer {
        name: String,
        player: String,
        game_stake: u64,
    },
    StartGame {
        name: String,
    },
    VoteForAdventure {
        name: String,
        player: String,
        adventure_number: u32,
        vote: u32,
    },
    GameAdventureStop {
        name: String,
        votes: Vec<GamePlayerVote>,
    },
    EndGame {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub games_count: i32,
}
