pub mod contract;
mod error;
pub mod helpers;
pub mod integration_tests;
pub mod msg;
pub mod state;

pub use crate::error::ContractError;

// use cosmwasm_std::{
//     entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
// };

// use serde::{Deserialize, Serialize};

// #[entry_point]
// pub fn instantiate(
//     _deps: DepsMut,
//     _env: Env,
//     _info: MessageInfo,
//     _msg: Empty,
// ) -> StdResult<Response> {
//     Ok(Response::new())
// }

// #[derive(Serialize, Deserialize)]
// struct QueryResp {
//     message: String,
// }

// #[entry_point]
// pub fn query(_deps: Deps, _env: Env, _msg: Empty) -> StdResult<Binary> {
//     let resp = QueryResp {
//         message: "Hello World".to_owned(),
//     };

//     to_binary(&resp)
// }
