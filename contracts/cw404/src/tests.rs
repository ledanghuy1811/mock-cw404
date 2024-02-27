use cosmwasm_std::{Addr, Uint128};
use cw20::{BalanceResponse, Cw20Coin};
use cw_multi_test::{App, ContractWrapper, Executor};

use cw404_package::TokenInfoResponse;

use crate::contract::{execute, instantiate, query};
use crate::msg::{InstantiateMsg, QueryMsg};

pub struct InstantiateResponse {
    pub app: App,
    pub address: Addr,
}

fn intantisate_contract(initial_balance_amount: Uint128) -> InstantiateResponse {
    let mut app = App::default();
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let address = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("admin"),
            &InstantiateMsg {
                name: "Orai Pandora".to_string(),
                symbol: "OPAN".to_string(),
                decimals: 6,
                initial_balances: vec![Cw20Coin {
                    address: "admin".to_string(),
                    amount: initial_balance_amount,
                }],
                admin: "admin".to_string(),
            },
            &[],
            "cw404 contract",
            None,
        )
        .unwrap();

    InstantiateResponse { app, address }
}

#[test]
pub fn initial_balance() {
    let instantiate_resp: InstantiateResponse = intantisate_contract(Uint128::from(10000u128));

    let resp: BalanceResponse = instantiate_resp
        .app
        .wrap()
        .query_wasm_smart(
            instantiate_resp.address,
            &QueryMsg::Balance {
                address: "admin".to_string(),
            },
        )
        .unwrap();
    assert_eq!(
        resp,
        BalanceResponse {
            balance: Uint128::from(10000u128)
        }
    );
}

#[test]
pub fn initial_token_info() {
    let instantiate_resp: InstantiateResponse = intantisate_contract(Uint128::from(10000u128));

    let resp: TokenInfoResponse = instantiate_resp
        .app
        .wrap()
        .query_wasm_smart(instantiate_resp.address, &QueryMsg::TokenInfo {})
        .unwrap();
    assert_eq!(
        resp,
        TokenInfoResponse {
            name: "Orai Pandora".to_string(),
            symbol: "OPAN".to_string(),
            decimals: 6,
            total_supply: Uint128::from(10000u128),
            admin: Addr::unchecked("admin"),
            units: Uint128::from(10u128.pow(6))
        }
    );
}
