#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg};
use crate::state::{GameState, MovieMagicContractState, QueryResp, State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:hackathon-movie-magic-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn instantiate2(
//     deps: DepsMut,
//     _env: Env,
//     info: MessageInfo,
//     msg: InstantiateMsg,
// ) -> Result<Response, ContractError> {
//     let state = State {
//         count: msg.count,
//         owner: info.sender.clone(),
//     };
//     set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
//     STATE.save(deps.storage, &state)?;

//     Ok(Response::new()
//         .add_attribute("method", "instantiate")
//         .add_attribute("owner", info.sender)
//         .add_attribute("count", msg.count.to_string()))
// }

// 1. Update the instantiation logic that allows games as empty array
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // let state = State {
    //     count: msg.count,
    //     owner: info.sender.clone(),
    // };
    let state = MovieMagicContractState {
        games: vec![],
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    // match msg {
    //     QueryMsg::GetCount {} => to_binary(&query::count(deps)?),
    // }
    // let resp = QueryResp {
    //     message: "Hello World".to_owned(),
    // };

    let resp: MovieMagicContractState = STATE.load(deps.storage)?;

    to_binary(&resp)
}

// 2. Support the invocation message to create the game with atleast 1 player

// 3. Support the invocation message so that another player can join the game with same name

// 4. Invocation to start the game

// 5. Invocation where adventure number, adventure vote of each player is sent

// 6. Invocation to end the game

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    use ExecuteMsg::*;

    match msg {
        InitGame {
            name,
            player,
            num_of_adventures,
            game_stake,
        } => exec::init_game(deps, name, player, num_of_adventures, game_stake),
        AddGamePlayer {
            name,
            player,
            game_stake,
        } => exec::add_player(deps, name, player, game_stake),
        StartGame { name } => exec::start_game(deps, name),
        VoteForAdventure {
            name,
            player,
            adventure_number,
            vote,
        } => exec::vote_for_adventure(deps, name, player, vote, adventure_number),
        GameAdventureStop {
            name,
            adventure_number,
        } => exec::game_adventure_stop(deps, name, adventure_number),
        EndGame {} => todo!(),
    }
}

mod exec {
    use std::collections::HashMap;

    use cosmwasm_std::StdError;

    use crate::msg::GamePlayerVote;

    use super::*;

    pub fn init_game(
        deps: DepsMut,
        name: String,
        player: String,
        num_of_adventures: u32,
        game_stake: u64,
    ) -> StdResult<Response> {
        let mut curr_games = STATE.load(deps.storage)?;
        let mut games = curr_games.games;
        let new_game = GameState {
            name: name.clone(),
            players: vec![deps.api.addr_validate(&player)?.to_string()],
            total_funds: game_stake,
            adventure_funds: 0,
            initiated: true,
            started: false,
            winner: "".to_string(),
            adventure_votes: vec![],
            num_of_adventures: num_of_adventures,
            adventureWinners: vec![],
            adventureRewards: vec![],
            adventureWinningVotes: vec![],
        };

        games.push(new_game);
        curr_games.games = games;

        STATE.save(deps.storage, &curr_games)?;

        Ok(Response::new())
    }

    pub fn add_player(
        deps: DepsMut,
        name: String,
        player: String,
        game_stake: u64,
    ) -> StdResult<Response> {
        let mut curr_games = STATE.load(deps.storage)?;
        let game_find_result = curr_games.games.iter_mut().find(|game| game.name == name);

        match game_find_result {
            Some(game) => {
                // Make sure the player is not already part of the game
                // Disable the below check after testing
                // if !game.players.contains(&player) {
                //     return Err(StdError::generic_err(
                //         "This player is already part of the game.",
                //     ));
                // }
                // Add the player to the game
                game.players
                    .push(deps.api.addr_validate(&player)?.to_string());
                // Add the funds from the player to the game
                game.total_funds += game_stake;
            }
            None => {
                return Err(StdError::generic_err("Game not found"));
            }
        }

        STATE.save(deps.storage, &curr_games)?;

        Ok(Response::new())
    }

    pub fn start_game(deps: DepsMut, name: String) -> StdResult<Response> {
        let mut curr_games: MovieMagicContractState = STATE.load(deps.storage)?;

        // Make sure there non zero about staked in the game pool

        let game_find_result = curr_games.games.iter_mut().find(|game| game.name == name);

        match game_find_result {
            Some(game) => {
                if (game.total_funds == 0) || (game.players.len() < 2) {
                    return Err(StdError::generic_err(
                        "Game cannot be started with less than 2 players or zero funds.",
                    ));
                } else {
                    game.started = true;
                    game.adventure_funds = ((80 * game.total_funds) / 100) as u64;
                    STATE.save(deps.storage, &curr_games)?;
                    Ok(Response::new())
                }
            }
            None => {
                return Err(StdError::generic_err("Game not found"));
            }
        }
    }

    pub fn vote_for_adventure(
        deps: DepsMut,
        name: String,
        player: String,
        vote: u32,
        adventure_number: u32,
    ) -> StdResult<Response> {
        let mut curr_games = STATE.load(deps.storage)?;

        let game_find_result = curr_games.games.iter_mut().find(|game| game.name == name);

        match game_find_result {
            Some(game) => {
                // Make sure the player is part of the game
                if !game.players.contains(&player) {
                    return Err(StdError::generic_err(
                        "This player is not part of the game.",
                    ));
                }
                // Make sure the game has been started
                if !game.started {
                    return Err(StdError::generic_err("Game has not been started yet."));
                }
                if (adventure_number < (game.adventure_votes.len() as u32)) {
                    // Add the vote of the player to the adventure
                    game.adventure_votes[adventure_number as usize].insert(player, vote);
                } else if (adventure_number == (game.adventure_votes.len() as u32)) {
                    let adventure_vote_hash_map: HashMap<String, u32> = HashMap::new();
                    game.adventure_votes.push(adventure_vote_hash_map);
                    // Add the vote of the player to the adventure
                    game.adventure_votes[adventure_number as usize].insert(player, vote);
                } else {
                    return Err(StdError::generic_err(
                        "Voting has not started for this adventure.",
                    ));
                }
            }
            None => {
                return Err(StdError::generic_err("Game not found"));
            }
        }

        STATE.save(deps.storage, &curr_games)?;

        Ok(Response::new())
    }

    pub fn game_adventure_stop(
        deps: DepsMut,
        name: String,
        adventure_stop_number: u32,
    ) -> StdResult<Response> {
        let mut curr_games = STATE.load(deps.storage)?;

        let game_find_result = curr_games.games.iter_mut().find(|game| game.name == name);

        match game_find_result {
            Some(game) => {
                // Get the adventure votes
                let adventure_votes = game.adventure_votes.get(adventure_stop_number as usize);
                let adventure_votes_unwrapped = adventure_votes.unwrap();
                // game.adventure_votes[adventure_stop_number as usize];

                let mut votes_count_map: HashMap<u32, Vec<String>> = HashMap::new();

                // Generate map to track who votes for given options
                for (player, vote) in adventure_votes_unwrapped.iter() {
                    if (votes_count_map.contains_key(vote)) {
                        let vec_ref = votes_count_map.get_mut(vote);
                        match vec_ref {
                            Some(vec) => {
                                vec.push(player.clone());
                            }
                            None => {
                                return Err(StdError::generic_err("Error in counting votes"));
                            }
                        }
                    } else {
                        votes_count_map.insert(vote.clone(), vec![player.clone()]);
                    }
                }

                let mut vote_count_opt1: u32 = 0;
                let mut vote_count_opt2: u32 = 0;

                // Count the number of votes given to each candidate option
                for (vote, players) in votes_count_map.iter() {
                    if (*vote == 1) {
                        vote_count_opt1 = players.len() as u32;
                    } else if (*vote == 2) {
                        vote_count_opt2 = players.len() as u32;
                    }
                }

                if (vote_count_opt1 > vote_count_opt2) {
                    // Calculate the reward amount for each player
                    let reward_amound = ((game.adventure_funds / (game.num_of_adventures as u64))
                        / (vote_count_opt1 as u64)) as u64;
                    game.adventureWinningVotes.push(vote_count_opt1);
                    game.adventureWinners
                        .push(votes_count_map.get(&1).unwrap().clone());
                    game.adventureRewards.push(reward_amound);
                } else {
                    // Calculate the reward amount for each player
                    let reward_amound = ((game.adventure_funds / (game.num_of_adventures as u64))
                        / (vote_count_opt2 as u64)) as u64;
                    game.adventureWinningVotes.push(vote_count_opt2);
                    game.adventureWinners
                        .push(votes_count_map.get(&2).unwrap().clone());
                    game.adventureRewards.push(reward_amound);
                }
            }
            None => {
                return Err(StdError::generic_err("Game not found"));
            }
        }

        STATE.save(deps.storage, &curr_games)?;

        Ok(Response::new())
    }
}

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn execute(
//     deps: DepsMut,
//     _env: Env,
//     info: MessageInfo,
//     msg: ExecuteMsg,
// ) -> Result<Response, ContractError> {
//     match msg {
//         ExecuteMsg::Increment {} => execute::increment(deps),
//         ExecuteMsg::Reset { count } => execute::reset(deps, info, count),
//     }
// }

// pub mod execute {
//     use super::*;

//     pub fn increment(deps: DepsMut) -> Result<Response, ContractError> {
//         STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
//             state.count += 1;
//             Ok(state)
//         })?;

//         Ok(Response::new().add_attribute("action", "increment"))
//     }

//     pub fn reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
//         STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
//             if info.sender != state.owner {
//                 return Err(ContractError::Unauthorized {});
//             }
//             state.count = count;
//             Ok(state)
//         })?;
//         Ok(Response::new().add_attribute("action", "reset"))
//     }
// }

// pub mod query {
//     use super::*;

//     pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
//         let state = STATE.load(deps.storage)?;
//         Ok(GetCountResponse { games_count: state.games.len() })
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
//     use cosmwasm_std::{coins, from_binary};

//     #[test]
//     fn proper_initialization() {
//         let mut deps = mock_dependencies();

//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(1000, "earth"));

//         // we can just call .unwrap() to assert this was a success
//         let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
//         assert_eq!(0, res.messages.len());

//         // it worked, let's query the state
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: GetCountResponse = from_binary(&res).unwrap();
//         assert_eq!(17, value.count);
//     }

//     #[test]
//     fn increment() {
//         let mut deps = mock_dependencies();

//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(2, "token"));
//         let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // beneficiary can release it
//         let info = mock_info("anyone", &coins(2, "token"));
//         let msg = ExecuteMsg::Increment {};
//         let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // should increase counter by 1
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: GetCountResponse = from_binary(&res).unwrap();
//         assert_eq!(18, value.count);
//     }

//     #[test]
//     fn reset() {
//         let mut deps = mock_dependencies();

//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(2, "token"));
//         let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // beneficiary can release it
//         let unauth_info = mock_info("anyone", &coins(2, "token"));
//         let msg = ExecuteMsg::Reset { count: 5 };
//         let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
//         match res {
//             Err(ContractError::Unauthorized {}) => {}
//             _ => panic!("Must return unauthorized error"),
//         }

//         // only the original creator can reset the counter
//         let auth_info = mock_info("creator", &coins(2, "token"));
//         let msg = ExecuteMsg::Reset { count: 5 };
//         let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

//         // should now be 5
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: GetCountResponse = from_binary(&res).unwrap();
//         assert_eq!(5, value.count);
//     }
// }
