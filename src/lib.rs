#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary,
    StdError,
    Binary,
    Deps,
    DepsMut,
    Env,
    MessageInfo,
    Response,
};
use cw_storage_plus::Item;

const COUNTER: Item<u8> = Item::new("value");

use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub enum CounterInitMsg {
    Zero,
    Set(u8)
}

#[cw_serde]
pub enum CounterExecMsg {
    Inc,
    Dec,
    Set(u8)
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum CounterQueryMsg {
    #[returns(CounterResponse)]
    Value
}

#[cw_serde]
pub struct CounterResponse {
    pub value: u8
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: CounterInitMsg,
) -> Result<Response, StdError> {
    COUNTER.save(
        deps.storage,
        &match msg {
            CounterInitMsg::Zero => 0,
            CounterInitMsg::Set(new_value) => new_value,
        }
    )?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: CounterExecMsg,
) -> Result<Response, StdError> {
    COUNTER.update::<_, StdError>(deps.storage, |old_value: u8| {
        Ok(match msg {
            CounterExecMsg::Inc => old_value.saturating_add(1),
            CounterExecMsg::Dec => old_value.saturating_sub(1),
            CounterExecMsg::Set(new_value) => new_value
        })
    })?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: CounterQueryMsg
) -> Result<Binary, StdError> {
    match msg {
        CounterQueryMsg::Value => Ok(to_json_binary(&CounterResponse {
            value: COUNTER.may_load(deps.storage)?.unwrap()
        })?)
    }
}
