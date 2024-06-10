module oaksecurity_cosmwasm_ctf_02 {
  
  import basicSpells.* from "./lib/basicSpells"
  import cw_types.* from "./lib/cw_types"
  import cw_utils.* from "./lib/cw_utils"
  import messaging.* from "./lib/messaging"
  import bank from "./lib/bank"

  
  var contract_state: ContractState
  var result: Result
  var bank: bank::Bank
  var time: int

  
  pure val CONTRACT_ADDRESS = "contract0"

  
  pure val ADDRESSES = Set("sender1", "sender2", "sender3", CONTRACT_ADDRESS)
  pure val DENOMS = Set("d1", "uawesome")
  pure val MAX_AMOUNT = 200


  type UserInfo = { total_tokens: BigInt, voting_power: BigInt, released_time: BigInt }
  type State = { system_state: SystemState, step_info: StepInfo }
  type SystemState = { voting_power: HashMap[str, UserInfo], block_time: BigInt }
  type StepInfo = { action_taken: str, msg_info: MsgInfo, msg_args: MsgArgs, action_successful: bool, action_error_description: str }
  type Coin = { denom: str, amount: BigInt }
  type DepositArgs = { deposit_amount: BigInt, sender: str }
  type WithdrawArgs = { withdraw_amount: BigInt, sender: str }
  type StakeArgs = { stake_amount: BigInt, sender: str }
  type UnstakeArgs = { unstake_amount: BigInt, sender: str }
  type MsgInfo = { sender: str, funds: List[Coin] }
  type Coin = { denom: str, amount: BigInt }
  type MsgInfo = { sender: str, funds: List[Coin] }
  type DepositArgs = { deposit_amount: BigInt, sender: str }
  type WithdrawArgs = { withdraw_amount: BigInt, sender: str }
  type StakeArgs = { stake_amount: BigInt, sender: str }
  type UnstakeArgs = { unstake_amount: BigInt, sender: str }
  type UserInfo = { total_tokens: BigInt, voting_power: BigInt, released_time: BigInt }
  type State = { voting_power: HashMap[str, UserInfo], block_time: BigInt }
  type StepInfo = { action_taken: str, msg_info: MsgInfo, msg_args: MsgArgs, action_successful: bool, action_error_description: str }
  type TraceState = { step_info: StepInfo, system_state: State }
  type InstantiateMsg = {  }
  type UserInfo = { total_tokens: int, voting_power: int, released_time: int }
  type MsgArgs =
    | MsgArgs_None
    | MsgArgs_DepositArgs(DepositArgs)
    | MsgArgs_WithdrawArgs(WithdrawArgs)
    | MsgArgs_StakeArgs(StakeArgs)
    | MsgArgs_UnstakeArgs(UnstakeArgs)
  type MsgArgs =
    | MsgArgs_None
    | MsgArgs_DepositArgs(DepositArgs)
    | MsgArgs_WithdrawArgs(WithdrawArgs)
    | MsgArgs_StakeArgs(StakeArgs)
    | MsgArgs_UnstakeArgs(UnstakeArgs)
  type ExecuteMsg =
    | ExecuteMsg_Deposit
    | ExecuteMsg_Withdraw({ amount: int })
    | ExecuteMsg_Stake({ lock_amount: int })
    | ExecuteMsg_Unstake({ unlock_amount: int })
  pure def instantiate(state: ContractState, _env: Env, _info: MessageInfo, _msg: InstantiateMsg): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body
    (Ok(Response_new), state)
  }

  pure def execute(state: ContractState, env: Env, info: MessageInfo, msg: ExecuteMsg): (Result[Response, ContractError], ContractState) = match msg {
    | ExecuteMsg_Deposit(__r) => deposit(state, info)
    | ExecuteMsg_Withdraw(__r) => withdraw(state, info, __r.amount)
    | ExecuteMsg_Stake(__r) => stake(state, env, info, __r.lock_amount)
    | ExecuteMsg_Unstake(__r) => unstake(state, env, info, __r.unlock_amount)
  }

  pure def deposit(state: ContractState, info: MessageInfo): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body
    (Ok(Response_new), state)
  }
                            
  action deposit_action = {
    // TODO: Change next line according to fund expectations
    pure val max_funds = MAX_AMOUNT
    
    pure val message: ExecuteMsg = ExecuteMsg_Deposit
    execute_message(message, max_funds)
  }

  pure def withdraw(state: ContractState, info: MessageInfo, amount: int): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body
    (Ok(Response_new), state)
  }
                            
  action withdraw_action = {
    // TODO: Change next line according to fund expectations
    pure val max_funds = MAX_AMOUNT
    nondet message_amount: int = 0.to(MAX_AMOUNT).oneOf()
    pure val message: ExecuteMsg = ExecuteMsg_Withdraw({ amount: message_amount })
    execute_message(message, max_funds)
  }

  pure def stake(state: ContractState, env: Env, info: MessageInfo, lock_amount: int): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body
    (Ok(Response_new), state)
  }
                            
  action stake_action = {
    // TODO: Change next line according to fund expectations
    pure val max_funds = MAX_AMOUNT
    nondet message_lock_amount: int = 0.to(MAX_AMOUNT).oneOf()
    pure val message: ExecuteMsg = ExecuteMsg_Stake({ lock_amount: message_lock_amount })
    execute_message(message, max_funds)
  }

  pure def unstake(state: ContractState, env: Env, info: MessageInfo, unlock_amount: int): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body
    (Ok(Response_new), state)
  }
                            
  action unstake_action = {
    // TODO: Change next line according to fund expectations
    pure val max_funds = MAX_AMOUNT
    nondet message_unlock_amount: int = 0.to(MAX_AMOUNT).oneOf()
    pure val message: ExecuteMsg = ExecuteMsg_Unstake({ unlock_amount: message_unlock_amount })
    execute_message(message, max_funds)
  }
  pure val DENOM = "testcoin"
  pure val LOCK_PERIOD = 60 * 60 * 24

  type ContractState = {
    voting_power: Addr -> UserInfo
  }

  pure val init_contract_state = {
    voting_power: Map()
  }

  action execute_step = all {
    any {
      deposit_action,
      withdraw_action,
      stake_action,
      unstake_action
    },
    advance_time,
  }

  pure def reply(state: ContractState, _env: Env, _reply: Reply): (Result, ContractState) = (Ok(Response_new), state)


  pure val init_bank_state = ADDRESSES.mapBy(_ => DENOMS.mapBy(_ => MAX_AMOUNT))

  val env_val = { block: { time: time } }

  action init = {
    // TODO: Change next line according to fund expectations
    pure val max_funds = 0

    nondet sender = Set("admin").oneOf()
    nondet denom = DENOMS.oneOf()
    nondet amount = 0.to(max_funds).oneOf()
    val funds = [{ denom: denom, amount: amount }]
    val info = { sender: sender, funds: funds }

    pure val message: InstantiateMsg = {  }
    pure val r = instantiate(init_contract_state, { block: { time: 0 } }, info, message)

    all {
      contract_state' = r._2,
      bank' = init_bank_state,
      result' = r._1,
      time' = 0,
    }
  }


  action execute_message(message, max_funds) = {
    nondet sender = ADDRESSES.oneOf()
    nondet denom = DENOMS.oneOf()
    nondet amount = 0.to(max_funds).oneOf()
    val funds = [{ denom: denom, amount: amount }]
    val info = { sender: sender, funds: funds }

    val r = execute(contract_state, env_val, info, message)
    all {
      bank.get(sender).get(denom) >= amount,
      match r._1 {
        | Ok(_) => bank' = bank.setBy(sender, balances => balances.setBy(denom, balance => balance - amount))
                               .setBy(CONTRACT_ADDRESS, balances => balances.setBy(denom, balance => balance + amount))
        | Err(_) => bank' = bank
      },
      result' = r._1,
      contract_state' = r._2,
    }
  }

  action advance_time = time' = time + 1

  action step = {
    val message_getting = get_message(result)
    val new_result = message_getting._1
    val opt_message = message_getting._2
    match opt_message {
      | Some(submsg) => {
          val current_state = { bank: bank, result: new_result, contract_state: contract_state }
          val new_state = process_message(current_state, env_val, CONTRACT_ADDRESS, submsg, reply)
          all {
            bank' = new_state.bank,
            result' = new_state.result,
            contract_state' = new_state.contract_state,
            advance_time,
          }
      }
      | None => execute_step
    }
  }

}
