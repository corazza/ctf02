
pub mod state_structs {
    use num_bigint::BigInt;
    use serde::Deserialize;
    use std::collections::HashMap;
    use itf::de::{self, As};
    
    #[derive(Clone, Debug, Deserialize)]
    pub struct Coin {
        pub denom: String,
        pub amount: BigInt
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct DepositArgs {
        pub deposit_amount: BigInt,
        pub sender: String
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct InstantiateMsg {
        
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct MsgInfo {
        pub sender: String,
        pub funds: Vec<Coin>
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct StakeArgs {
        pub stake_amount: BigInt,
        pub sender: String
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct State {
        pub voting_power: HashMap[str, UserInfo],
        pub block_time: BigInt
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct StepInfo {
        pub action_taken: String,
        pub msg_info: MsgInfo,
        pub msg_args: MsgArgs,
        pub action_successful: bool,
        pub action_error_description: String
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct SystemState {
        pub voting_power: HashMap[str, UserInfo],
        pub block_time: BigInt
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct TraceState {
        pub step_info: StepInfo,
        pub system_state: State
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct UnstakeArgs {
        pub unstake_amount: BigInt,
        pub sender: String
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct UserInfo {
        pub total_tokens: BigInt,
        pub voting_power: BigInt,
        pub released_time: BigInt
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct WithdrawArgs {
        pub withdraw_amount: BigInt,
        pub sender: String
    }
    
    #[derive(Clone, Debug, Deserialize)]
    pub struct ContractState {
        pub voting_power: HashMap<String, UserInfo>
    }
    
    #[derive(Clone, Debug, Deserialize)]
    pub struct NondetPicks {
        
        #[serde(with = "As::<de::Option::<_>>")]
        pub sender: Option<String>,
        
        #[serde(with = "As::<de::Option::<_>>")]
        pub denom: Option<String>,
        
        #[serde(with = "As::<de::Option::<_>>")]
        pub amount: Option<BigInt>,
        
        #[serde(with = "As::<de::Option::<_>>")]
        pub funds_denom: Option<String>,
        
        #[serde(with = "As::<de::Option::<_>>")]
        pub funds_amount: Option<BigInt>,
        
        #[serde(with = "As::<de::Option::<_>>")]
        pub funds_denom: Option<String>,
        
        #[serde(with = "As::<de::Option::<_>>")]
        pub funds_amount: Option<BigInt>,
        
        #[serde(with = "As::<de::Option::<_>>")]
        pub msg_info_sender: Option<String>,
        
        #[serde(with = "As::<de::Option::<_>>")]
        pub step_info_action_taken: Option<String>,
        
        #[serde(with = "As::<de::Option::<_>>")]
        pub step_info_msg_args: Option<MsgArgs>,
        
        #[serde(with = "As::<de::Option::<_>>")]
        pub step_info_action_successful: Option<bool>,
        
        #[serde(with = "As::<de::Option::<_>>")]
        pub step_info_action_error_description: Option<String>,
        
        #[serde(with = "As::<de::Option::<_>>")]
        pub system_state_voting_power: Option<HashMap[str, UserInfo]>,
        
        #[serde(with = "As::<de::Option::<_>>")]
        pub system_state_block_time: Option<BigInt>,
        
        #[serde(with = "As::<de::Option::<_>>")]
        pub message_amount: Option<BigInt>,
        
        #[serde(with = "As::<de::Option::<_>>")]
        pub message_lock_amount: Option<BigInt>,
        
        #[serde(with = "As::<de::Option::<_>>")]
        pub message_unlock_amount: Option<BigInt>
    }
    
    #[derive(Clone, Debug, Deserialize)]
    pub struct Message {}
    #[derive(Clone, Debug, Deserialize)]
    pub struct Attribute {
        pub key: String,
        pub value: QuintSerializedValue,
    }

    #[derive(Clone, Debug, Deserialize)]
    #[serde(tag = "tag", content = "value")]
    pub enum QuintSerializedValue {
        FromInt(BigInt),
        FromStr(String),
        FromListInt(Vec<BigInt>),
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Response {
        pub messages: Vec<Message>,
        pub attributes: Vec<Attribute>,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct State {
        pub contract_state: ContractState,
        pub bank: HashMap<String, HashMap<String, BigInt>>,
        #[serde(with = "As::<de::Result::<_, _>>")]
        pub result: Result<Response, String>,
        pub action_taken: String,
        pub nondet_picks: NondetPicks,
        pub time: BigInt,
    }

}
    
#[cfg(test)]
pub mod tests {
    use oaksecurity_cosmwasm_ctf_02::contract;
    use oaksecurity_cosmwasm_ctf_02::msg::{ExecuteMsg, InstantiateMsg};


    use crate::state_structs::*;
    use cosmwasm_std::{coin, Addr, Uint128};
    use cw_multi_test::{App, AppResponse, ContractWrapper, Executor};
    use itf::trace_from_str;
    use num_bigint::BigInt;
    use num_traits::{ToPrimitive, Zero};

    pub const DENOM: &str = "uawesome";
    pub const TICK: u64 = 1;

    pub fn mint_tokens(mut app: App, recipient: String, denom: String, amount: Uint128) -> App {
        app.sudo(cw_multi_test::SudoMsg::Bank(
            cw_multi_test::BankSudo::Mint {
                to_address: recipient.to_owned(),
                amount: vec![coin(amount.u128(), denom)],
            },
        ))
        .unwrap();
        app
    }

    fn compare_state(test_state: &TestState, app: &App, state: &State) {
        // compare contract balances
        let balance = app
            .wrap()
            .query_balance(&test_state.contract_addr, DENOM)
            .unwrap()
            .amount;
        let trace_balance = state
            .bank
            .get(&test_state.contract_addr.to_string())
            .and_then(|x| x.get(DENOM))
            .and_then(|x| x.to_u128())
            .unwrap_or(0);
        println!(
            "Contract balance ({:?}) for {DENOM}: {:?} vs {:?}",
            test_state.contract_addr,
            balance,
            Uint128::new(trace_balance)
        );
        assert_eq!(balance, Uint128::new(trace_balance));

        // TODO: Query the contract and compare the state as you wish
    }

    fn compare_result(
        trace_result: Result<Response, String>,
        app_result: Result<AppResponse, anyhow::Error>,
    ) {
        if trace_result.is_ok() {
            assert!(
                app_result.is_ok(),
                "Action unexpectedly failed, error: {:?}",
                app_result.err()
            );
            println!("Action successful as expected");
        } else {
            assert!(
                app_result.is_err(),
                "Expected action to fail with error: {:?}",
                trace_result.err()
            );
            println!("Action failed as expected");
        }
    }

    fn funds_from_trace(amount: Option<BigInt>, denom: Option<String>) -> Vec<cosmwasm_std::Coin> {
        if amount.is_none() || denom.is_none() || amount == Some(Zero::zero()) {
            return vec![];
        }

        vec![coin(
            amount.as_ref().unwrap().to_u128().unwrap(),
            denom.unwrap(),
        )]
    }

    // Testing is stateful.
    struct TestState {
        // we will only know the contract address once we have processed an `instantiate` step
        pub contract_addr: Addr,
    }

    #[test]
    fn model_test() {
        let mut app = App::default();
        let code = ContractWrapper::new(contract::execute, contract::instantiate, contract::query);
        let code_id = app.store_code(Box::new(code));

        // create test state
        let mut test_state = TestState {
            contract_addr: Addr::unchecked("contract0"),
        };

        // load trace data
        let data = include_str!("../quint/test.itf.json");
        let trace: itf::Trace<State> = trace_from_str(data).unwrap();

        for s in trace.states {
            let last_result = s.value.result.clone();
            if last_result.is_ok() && !last_result.unwrap().messages.is_empty() {
                println!("Processing messages, skipping");
                continue;
            }

            let action_taken = &s.value.action_taken;
            let nondet_picks = &s.value.nondet_picks;
            let amount = nondet_picks.amount.clone();
            let denom = nondet_picks.denom.clone();
            let sender = nondet_picks.sender.clone();

            println!("Step number: {:?}", s.meta.index);
            println!("Result from trace: {:?}", s.value.result.clone());

            match action_taken.as_str() {


               "deposit_action" => {
                    let sender = Addr::unchecked(sender.unwrap());
                    let funds = funds_from_trace(amount, denom);

                    let msg = ExecuteMsg::Deposit {  };
                    println!("Message: {:?}", msg);
                    println!("Sender: {:?}", sender);
                    println!("Funds: {:?}", funds);

                    let res = app.execute_contract(
                        sender,
                        test_state.contract_addr.clone(),
                        &msg,
                        &funds,
                    );

                    compare_result(s.value.result.clone(), res)
                }


               "q::init" => {
                    println!("Initializing contract.");

                    let sender = Addr::unchecked(sender.unwrap());
                    let funds = funds_from_trace(amount, denom);

                    let msg = InstantiateMsg {  };
                    println!("Message: {:?}", msg);
                    println!("Sender: {:?}", sender);
                    println!("Funds: {:?}", funds);

                    test_state.contract_addr = app.instantiate_contract(
                        code_id,
                        sender,
                        &msg,
                        &funds,
                        "test",
                        None,
                    ).unwrap();

                    for (addr, coins) in s.value.bank.clone().iter() {
                        for (denom, amount) in coins.iter() {
                            app = mint_tokens(
                                app,
                                addr.clone(),
                                denom.to_string(),
                                Uint128::new(amount.to_u128().unwrap()),
                            );
                        }
                    }

               }


               "stake_action" => {
                    let sender = Addr::unchecked(sender.unwrap());
                    let funds = funds_from_trace(amount, denom);

                    let message_lock_amount = nondet_picks.message_lock_amount.clone().unwrap().to_u64().unwrap().into();
                    let msg = ExecuteMsg::Stake { lock_amount: message_lock_amount };
                    println!("Message: {:?}", msg);
                    println!("Sender: {:?}", sender);
                    println!("Funds: {:?}", funds);

                    let res = app.execute_contract(
                        sender,
                        test_state.contract_addr.clone(),
                        &msg,
                        &funds,
                    );

                    compare_result(s.value.result.clone(), res)
                }


               "unstake_action" => {
                    let sender = Addr::unchecked(sender.unwrap());
                    let funds = funds_from_trace(amount, denom);

                    let message_unlock_amount = nondet_picks.message_unlock_amount.clone().unwrap().to_u64().unwrap().into();
                    let msg = ExecuteMsg::Unstake { unlock_amount: message_unlock_amount };
                    println!("Message: {:?}", msg);
                    println!("Sender: {:?}", sender);
                    println!("Funds: {:?}", funds);

                    let res = app.execute_contract(
                        sender,
                        test_state.contract_addr.clone(),
                        &msg,
                        &funds,
                    );

                    compare_result(s.value.result.clone(), res)
                }


               "withdraw_action" => {
                    let sender = Addr::unchecked(sender.unwrap());
                    let funds = funds_from_trace(amount, denom);

                    let message_amount = nondet_picks.message_amount.clone().unwrap().to_u64().unwrap().into();
                    let msg = ExecuteMsg::Withdraw { amount: message_amount };
                    println!("Message: {:?}", msg);
                    println!("Sender: {:?}", sender);
                    println!("Funds: {:?}", funds);

                    let res = app.execute_contract(
                        sender,
                        test_state.contract_addr.clone(),
                        &msg,
                        &funds,
                    );

                    compare_result(s.value.result.clone(), res)
                }

                _ => panic!("Invalid action taken"),
            }
            compare_state(&test_state, &app, &(s.value.clone()));
            println!(
                "clock is advancing for {} seconds",
                TICK
            );
            app.update_block(|block| {
                block.time = block.time.plus_seconds(TICK);
            });
            println!("-----------------------------------");
        }
    }
}
