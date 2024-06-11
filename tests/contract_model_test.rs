mod state_structs {

    use num_bigint::BigInt;
    use serde::Deserialize;
    use std::collections::HashMap;

    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UserInfo {
        pub total_tokens: BigInt,
        pub voting_power: BigInt,
        pub released_time: BigInt
    }

    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct State {
        pub system_state: SystemState,
        pub step_info: StepInfo,
    }

    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SystemState {
        pub voting_power: HashMap<String, UserInfo>,
        pub block_time: BigInt
    }

    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StepInfo {
        pub action_taken: String,
        pub msg_info: MsgInfo,
        pub msg_args: MsgArgs,
        pub action_successful: bool,
        pub action_error_description: String,

        
    }

    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Coin {
        pub denom: String,
        pub amount: BigInt,
    }

    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DepositArgs {
        pub deposit_amount: BigInt,
        pub sender: String
    }

    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WithdrawArgs {
        pub withdraw_amount: BigInt,
        pub sender: String
    }

    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StakeArgs {
        pub stake_amount: BigInt,
        pub sender: String
    }

    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UnstakeArgs {
        pub unstake_amount: BigInt,
        pub sender: String
    }

    #[derive(Clone, Debug, Deserialize)]
    #[serde(tag = "tag", content = "value")]
    pub enum MsgArgs {
        None,
        DepositArgs(DepositArgs),
        WithdrawArgs(WithdrawArgs),
        StakeArgs(StakeArgs),
        UnstakeArgs(UnstakeArgs)


    }

    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct MsgInfo {
        pub sender: String,
        pub funds: Vec<Coin>,
    }    
}


#[cfg(test)]
mod contract_model_tests {
    use cosmwasm_std::{coin, StdError};
    use cosmwasm_std::{Addr, Uint128};
    use cw_multi_test::{App, ContractWrapper, Executor, AppResponse};
    use itf::trace_from_str;
    use serde::de::value::Error;

    use crate::contract_model_test::state_structs::*;
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
    use crate::contract::{DENOM, LOCK_PERIOD};
    use num_traits::ToPrimitive;

    // admin only instantiates the contract, no privileged role
    pub const ADMIN: &str = "admin";

    pub const USERS_ALLOWANCE: u128 = 1_000_000;
    // constants for users
    pub const USER_A: &str = "user_a";
    pub const USER_B: &str = "user_b";
    pub const USER_C: &str = "user_c";

    struct TestState {
        // we will only know the contract address once we have processed an `instantiate` step
        pub contract_addr: Addr,
    }

    // a function to compare the ITF state and the contract state
    fn compare_state(test_state: &TestState, app: &App, state: &SystemState) {
        // for each user, check its info using the query
        for user in state.voting_power.keys() {
            // create a message to query the user info
            let msg = QueryMsg::GetUser {
                user: user.clone(),
            };
            let user_info: crate::state::UserInfo = app
                .wrap()
                .query_wasm_smart(test_state.contract_addr.clone(), &msg)
                .unwrap();
                        

            // check that total tokens and voting power match.
            // no need to check the release time, because the modelling and the contract don't necessarily match
            // (modelling uses discrete time, akin to block times, but the contract uses timestamps)
            println!("{:?}", user_info);
            println!("{:?}", state.voting_power[user]);
            assert_eq!(user_info.total_tokens.u128(), state.voting_power[user].total_tokens.to_u128().unwrap());
            assert_eq!(user_info.voting_power, state.voting_power[user].voting_power.to_u128().unwrap());
            
        }
    }

    pub fn mint_tokens(mut app: App, recipient: String, amount: Uint128) -> App {
        app.sudo(cw_multi_test::SudoMsg::Bank(
            cw_multi_test::BankSudo::Mint {
                to_address: recipient.to_owned(),
                amount: vec![coin(amount.u128(), DENOM)],
            },
        ))
        .unwrap();
        app
    }

    fn deposit_funds(mut app: App,test_state: &TestState, user: Addr, amount: Uint128) -> (App, bool) {
        let msg = ExecuteMsg::Deposit {};
        let success = app.execute_contract(
            user,
            test_state.contract_addr.clone(),
            &msg,
            &[coin(amount.u128(), DENOM)],
        ).is_ok();
        
        (app, success)
    }

    #[test]
    fn test_execute_itf(){
        let mut app = App::default();
        let code = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        let code_id = app.store_code(Box::new(code));

        // initialize the state with the empty address for now
        let mut test_state = TestState {
            contract_addr: Addr::unchecked(""),
        };

        // now we can load trace data        
        let data = include_str!("../../../quint/ctf-02/out.itf.json");
        let trace: itf::Trace<State> = trace_from_str(data).unwrap();

        for s in trace.states {
            let step_info = &s.value.step_info;
            print!("\n\n");
            // print debug info
            println!("action taken: {}", step_info.action_taken);
            match step_info.action_taken.as_str() {
                "init" => {
                    println!("Initializing the contract");
                    let msg = InstantiateMsg {};
                    test_state.contract_addr = app
                        .instantiate_contract(
                            code_id, 
                            Addr::unchecked(ADMIN), 
                            &msg, 
                            &[], 
                            "test init", 
                            None).unwrap();

                    // now we want to let all users have sufficient funds
                    app = mint_tokens(app, USER_A.to_string(), Uint128::new(USERS_ALLOWANCE));
                    app = mint_tokens(app, USER_B.to_string(), Uint128::new(USERS_ALLOWANCE));
                    app = mint_tokens(app, USER_C.to_string(), Uint128::new(USERS_ALLOWANCE));

                    // the next step is getting them initially deposited funds (there is no assumption
                    // in the model that they start from 0)
                    (app, _) = deposit_funds(
                        app, 
                        &test_state, 
                        Addr::unchecked(USER_A), 
                        Uint128::new(s.value.system_state.voting_power[USER_A].total_tokens.to_u128().unwrap())
                    );

                    (app, _) = deposit_funds(
                        app, 
                        &test_state, 
                        Addr::unchecked(USER_B), 
                        Uint128::new(s.value.system_state.voting_power[USER_B].total_tokens.to_u128().unwrap())
                    );

                    (app, _) = deposit_funds(
                        app, 
                        &test_state, 
                        Addr::unchecked(USER_C), 
                        Uint128::new(s.value.system_state.voting_power[USER_C].total_tokens.to_u128().unwrap())
                    );
                    


                },
                "advance_time" => {
                    println!(
                        "advancing clock for {} seconds (LOCK_PERIOD)",
                        LOCK_PERIOD
                    );

                    // fast forward LOCK_PERIOD seconds
                    app.update_block(|block| {
                        block.time = block.time.plus_seconds(LOCK_PERIOD);
                    });

                },
                
                "deposit" => {
                    if let MsgArgs::DepositArgs(deposit_args) = &step_info.msg_args {
                        println!("Depositing {} tokens for {}", deposit_args.deposit_amount, deposit_args.sender);
                        
                        let mut success = true;
                        (app, success) = deposit_funds(
                            app, 
                            &test_state, 
                            Addr::unchecked(deposit_args.sender.clone()), 
                            Uint128::new(deposit_args.deposit_amount.to_u128().unwrap())
                        );
                        if step_info.action_successful {
                            println!("Deposit successful");
                            assert!(success);                            
                        } else {
                            println!("Deposit failed");
                            assert!(!success);
                        }
                        assert_eq!(success, step_info.action_successful);

                    } else {
                        println!("DEPOSIT: Wrong message arguments");
                        assert!(false);
                    }
                },
                "withdraw" => {
                    if let MsgArgs::WithdrawArgs(withdraw_args) = &step_info.msg_args {
                        println!("Withdrawing {} tokens for {}", withdraw_args.withdraw_amount, withdraw_args.sender);
                        let msg = ExecuteMsg::Withdraw {
                            amount: Uint128::new(withdraw_args.withdraw_amount.to_u128().unwrap()),
                        };
                        let res = app.execute_contract(
                            Addr::unchecked(withdraw_args.sender.clone()),
                            test_state.contract_addr.clone(),
                            &msg,
                            &[],
                        );
                        if step_info.action_successful {
                            println!("Withdraw successful");
                            assert!(res.is_ok());                            
                        } else {
                            println!("Withdraw failed");
                            assert!(res.is_err());
                            
                        }
                        
                    } else {
                        println!("WITHDRAW: Wrong message arguments");
                        assert!(false);
                    }                
                },
                "stake" => {
                    if let MsgArgs::StakeArgs(stake_args) = &step_info.msg_args {
                        println!("Staking {} tokens for {}", stake_args.stake_amount, stake_args.sender);
                        let msg = ExecuteMsg::Stake {
                            lock_amount: stake_args.stake_amount.to_u128().unwrap(),
                        };
                        let res = app.execute_contract(
                            Addr::unchecked(stake_args.sender.clone()),
                            test_state.contract_addr.clone(),
                            &msg,
                            &[],
                        );
                        if step_info.action_successful {
                            println!("Stake successful");
                            assert!(res.is_ok());                            
                        } else {
                            println!("Stake failed");
                            assert!(res.is_err());
                        }
                        
                    } else {
                        println!("STAKE: Wrong message arguments");
                        assert!(false);
                    }                                
                },
                "unstake" => {
                    if let MsgArgs::UnstakeArgs(unstake_args) = &step_info.msg_args {
                        println!("Unstaking {} tokens for {}", unstake_args.unstake_amount, unstake_args.sender);
                        let msg = ExecuteMsg::Unstake {
                            unlock_amount: unstake_args.unstake_amount.to_u128().unwrap(),
                        };
                        let res = app.execute_contract(
                            Addr::unchecked(unstake_args.sender.clone()),
                            test_state.contract_addr.clone(),
                            &msg,
                            &[],
                        );
                        if step_info.action_successful {
                            println!("Unstake successful");
                            assert!(res.is_ok());                            
                        } else {
                            println!("Unstake failed");
                            assert!(res.is_err());
                        }
                    } else {
                        println!("UNSTAKE: Wrong message arguments");
                        assert!(false);
                    }                                                                
                },
                unknown_action_taken => {
                    println!("Unknown action: {}", unknown_action_taken);
                    assert!(false);
                }
                
            }
            // whichever step we took, now we compare the state
            compare_state(&test_state, &app, &s.value.system_state);
        }

        assert!(true);
    }
}