mod state_structs {
    use num_bigint::BigInt;
    use serde::Deserialize;
    use std::collections::HashMap;

    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Coin {
        pub denom: String,
        pub amount: BigInt,
    }

    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct MsgInfo {
        pub sender: String,
        pub funds: Vec<Coin>,
    }

    #[derive(Clone, Debug, Deserialize)]
    #[serde(tag = "tag", content = "value")]
    pub enum MsgArgs {
        None,
        DepositArgs(DepositArgs),
        WithdrawArgs(WithdrawArgs),
        StakeArgs(StakeArgs),
        UnstakeArgs(UnstakeArgs),
    }

    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DepositArgs {
        pub deposit_amount: BigInt,
        pub sender: String,
    }

    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WithdrawArgs {
        pub withdraw_amount: BigInt,
        pub sender: String,
    }

    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StakeArgs {
        pub stake_amount: BigInt,
        pub sender: String,
    }

    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UnstakeArgs {
        pub unstake_amount: BigInt,
        pub sender: String,
    }

    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UserInfo {
        pub total_tokens: BigInt,
        pub voting_power: BigInt,
        pub released_time: BigInt,
    }

    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct State {
        pub voting_power: HashMap<String, UserInfo>,
        pub block_time: BigInt,
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
    pub struct TraceState {
        pub step_info: StepInfo,
        pub system_state: State,
    }
}

#[cfg(test)]
mod model_tests_generated {
    use cosmwasm_std::{coin, BankMsg, CosmosMsg};
    use cosmwasm_std::{Addr, Uint128};
    use cw_multi_test::{App, ContractWrapper, Executor};
    use itf::trace_from_str;
    use num_traits::ToPrimitive;

    use crate::contract_model_test_generated::state_structs::*;
    use crate::msg::*;
    use crate::ContractError;
    use crate::integration_tests::tests::ADMIN;

    pub const MAX_AMOUNT: u128 = 500;
    pub const NOERROR: &str = "no error";
    pub const UNSTAKE_INSUFFICIENT_FUNDS: &str = "unstake: insufficient funds";
    pub const UNSTAKE_LOCKED: &str = "unstake: locked";
    pub const STAKE_INSUFFICIENT_FUNDS: &str = "stake: insufficient funds";
    pub const WITHDRAW_INSUFFICIENT_FUNDS: &str = "withdraw: insufficient funds";
    pub const ACTION_STAKE: &str = "stake";
    pub const ACTION_UNSTAKE: &str = "unstake";
    pub const ACTION_WITHDRAW: &str = "withdraw";
    pub const ACTION_DEPOSIT: &str = "deposit";
    pub const ACTION_ADVANCE_TIME: &str = "advance_time";
    pub const DENOM: &str = "testcoin";
    pub const LOCK_PERIOD: u64 = 60 * 60 * 24; // One day
    
    // Testing is stateful.
    struct TestState {
        // we will only know the contract address once we have processed an `instantiate` step
        pub contract_addr: Addr,
    }

    fn trace_coin_to_cw_coin(trace_coin: &Coin) -> cosmwasm_std::Coin {
        let amount = trace_coin.amount.to_u128().unwrap();
        cosmwasm_std::Coin {
            denom: DENOM.to_owned(),
            amount: Uint128::new(amount),
        }
    }

    
    fn compare_state(test_state: &TestState, app: &App, state: &TraceState) {
        // Compare voting power and user info for each user in the trace
        for (user, trace_user_info) in &state.system_state.voting_power {
            // Query the contract for user info
            let user_info: crate::state::UserInfo = app
                .wrap()
                .query_wasm_smart(
                    test_state.contract_addr.clone(),
                    &QueryMsg::GetUser {
                        user: user.clone(),
                    },
                )
                .unwrap();

            // Compare total tokens
            let trace_total_tokens = trace_user_info.total_tokens.to_u128().unwrap();
            assert_eq!(user_info.total_tokens, Uint128::new(trace_total_tokens));

            // Compare voting power
            let trace_voting_power = trace_user_info.voting_power.to_u128().unwrap();
            assert_eq!(user_info.voting_power, trace_voting_power);

            // Compare released time
            let trace_released_time = trace_user_info.released_time.to_u64().unwrap();
            assert_eq!(user_info.released_time.seconds(), trace_released_time);
        }

        // Compare voting power for each user in the trace using the GetVotingPower query
        for (user, trace_user_info) in &state.system_state.voting_power {
            // Query the contract for voting power
            let voting_power: u128 = app
                .wrap()
                .query_wasm_smart(
                    test_state.contract_addr.clone(),
                    &QueryMsg::GetVotingPower {
                        user: user.clone(),
                    },
                )
                .unwrap();

            // Compare voting power
            let trace_voting_power = trace_user_info.voting_power.to_u128().unwrap();
            assert_eq!(voting_power, trace_voting_power);
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

    #[test]
    fn test_execute_json_generated() {
        let mut app = App::default();
        let code = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        let code_id = app.store_code(Box::new(code));

        // create test state
        let mut test_state = TestState {
            contract_addr: Addr::unchecked(""), // set in init step
        };

        // load trace data
        let data = include_str!("../../../quint/ctf-02/out.itf.json");
        let trace: itf::Trace<TraceState> = trace_from_str(data).unwrap();

        for s in trace.states {
            let step_info = &s.value.step_info;
            let sender = &step_info.msg_info.sender;

            match step_info.action_taken.as_str() {
                "init" => {
                    // this arm corresponds to the initialization of the contract state based on the trace
                    println!("Initializing contract.");

                    // init contract
                    let msg = InstantiateMsg {};
                    test_state.contract_addr = app
                        .instantiate_contract(
                            code_id,
                            Addr::unchecked(ADMIN),
                            &msg,
                            &[],
                            "test",
                            None,
                        )
                        .unwrap();

                    // Initialize users' voting power based on the trace
                    let user_a = Addr::unchecked("user_a");
                    let user_b = Addr::unchecked("user_b");
                    let user_c = Addr::unchecked("user_c");

                    // Set initial tokens for users
                    let initial_tokens = vec![
                        (user_a.clone(), Uint128::from(100u128)),
                        (user_b.clone(), Uint128::from(10u128)),
                        (user_c.clone(), Uint128::from(300u128)),
                    ];

                    for (user, amount) in initial_tokens {
                        app = mint_tokens(app, user.to_string(), amount);
                        app.execute_contract(
                            user.clone(),
                            test_state.contract_addr.clone(),
                            &ExecuteMsg::Deposit {},
                            &[coin(amount.u128(), DENOM)],
                        )
                        .unwrap();
                    }

                    compare_state(&test_state, &app, &s.value);
                }
                "stake" => {
                    // get the amount to stake from the trace
                    let amount: Uint128 =
                        Uint128::new(match &step_info.msg_args {
                            MsgArgs::StakeArgs(args) => args.stake_amount.to_u128().unwrap(),
                            _ => panic!("Expected StakeArgs in msg_args"),
                        });

                    println!("user {} staking {}", sender, amount);

                    // send the Stake message
                    let msg = ExecuteMsg::Stake {
                        lock_amount: amount.u128(),
                    };
                    let res = app.execute_contract(
                        Addr::unchecked(sender),
                        test_state.contract_addr.to_owned(),
                        &msg,
                        &[],
                    );

                    if !step_info.action_successful {
                        println!("error: {:?}", step_info.action_error_description);
                        let returned_err: ContractError = res.unwrap_err().downcast().unwrap();
                        match returned_err {
                            ContractError::InsufficientFunds {} => {
                                assert_eq!(step_info.action_error_description, STAKE_INSUFFICIENT_FUNDS)
                            }
                            err => {
                                println!("unknown contract error: {:?}", err);
                                assert!(false);
                            }
                        }
                    } else {
                        res.unwrap();
                    }

                    compare_state(&test_state, &app, &s.value);
                }
                "advance_time" => {
                    println!("Advancing time by {} seconds", LOCK_PERIOD);

                    // fast forward LOCK_PERIOD seconds
                    app.update_block(|block| {
                        block.time = block.time.plus_seconds(LOCK_PERIOD);
                    });

                    compare_state(&test_state, &app, &s.value);
                }
                "withdraw" => {
                    // get the amount the user wants to withdraw
                    let amount: Uint128 =
                        Uint128::new(match &step_info.msg_args {
                            MsgArgs::WithdrawArgs(args) => args.withdraw_amount.to_u128().unwrap(),
                            _ => panic!("Unexpected msg_args for withdraw action"),
                        });

                    println!("user {} withdrawing {}", sender, amount);

                    // send the withdrawal message
                    let msg = ExecuteMsg::Withdraw { amount };
                    let res = app.execute_contract(
                        Addr::unchecked(sender),
                        test_state.contract_addr.to_owned(),
                        &msg,
                        &[],
                    );

                    if !step_info.action_successful {
                        println!("error: {:?}", step_info.action_error_description);
                        let returned_err: ContractError = res.unwrap_err().downcast().unwrap();
                        match returned_err {
                            ContractError::InsufficientFunds {} => {
                                assert_eq!(step_info.action_error_description, "withdraw: insufficient funds")
                            }
                            err => {
                                println!("unknown contract error: {:?}", err);
                                assert!(false);
                            }
                        }
                    } else {
                        res.unwrap();
                    }

                    compare_state(&test_state, &app, &s.value);
                }
                "unstake" => {
                    // get the unstake amount from the trace
                    let amount: Uint128 =
                        Uint128::new(match &step_info.msg_args {
                            MsgArgs::UnstakeArgs(args) => args.unstake_amount.to_u128().unwrap(),
                            _ => panic!("Expected UnstakeArgs in msg_args"),
                        });

                    println!("user {} unstaking {}", sender, amount);

                    // send the Unstake message
                    let msg = ExecuteMsg::Unstake {
                        unlock_amount: amount.u128(),
                    };
                    let res = app.execute_contract(
                        Addr::unchecked(sender),
                        test_state.contract_addr.to_owned(),
                        &msg,
                        &[],
                    );

                    if (!step_info.action_successful) {
                        println!("error: {:?}", step_info.action_error_description);
                        let returned_err: ContractError = res.unwrap_err().downcast().unwrap();
                        match returned_err {
                            ContractError::InsufficientVotingPower {} => {
                                assert_eq!(step_info.action_error_description, UNSTAKE_INSUFFICIENT_FUNDS)
                            }
                            err => {
                                println!("unknown contract error: {:?}", err);
                                assert!(false);
                            }
                        }
                    } else {
                        res.unwrap();
                    }

                    compare_state(&test_state, &app, &s.value);
                }
                "deposit" => {
                    // Extract the deposit amount from the trace
                    if let MsgArgs::DepositArgs(deposit_args) = &step_info.msg_args {
                        let deposit_amount: Uint128 =
                            Uint128::new(deposit_args.deposit_amount.to_u128().unwrap());

                        println!(
                            "user {} depositing {} {}",
                            sender, deposit_amount, DENOM
                        );

                        // Mint tokens to the user's account before depositing
                        app = mint_tokens(app, sender.clone(), deposit_amount);

                        // Prepare the funds to be sent with the deposit message
                        let funds = vec![cosmwasm_std::Coin {
                            denom: DENOM.to_string(),
                            amount: deposit_amount,
                        }];

                        // Send the deposit message
                        let msg = ExecuteMsg::Deposit {};
                        let res = app.execute_contract(
                            Addr::unchecked(sender),
                            test_state.contract_addr.to_owned(),
                            &msg,
                            &funds,
                        );

                        if step_info.action_successful {
                            res.unwrap();
                        } else {
                            println!("error: {:?}", step_info.action_error_description);
                            res.unwrap_err();
                        }

                        compare_state(&test_state, &app, &s.value);
                    } else {
                        panic!("Unexpected MsgArgs for deposit action");
                    }
                }
                unknown_action_taken => {
                    println!("Unknown action: {}", unknown_action_taken);
                    assert!(false);
                }
            }
        }
    }
}