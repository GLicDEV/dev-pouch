use std::time::{SystemTime, UNIX_EPOCH};

use candid::{CandidType, Encode};

use ic_agent::Agent;
use ic_types::Principal;
use ic_utils::call::SyncCall;
use ic_utils::Canister;

pub const DEFAULT_IC_GATEWAY: &str = "https://ic0.app";

/// Id of the ledger canister on the IC.
pub const MAINNET_LEDGER_CANISTER_ID: Principal =
    Principal::from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x01, 0x01]);

const ACCOUNT_BALANCE_METHOD: &str = "account_balance_dfx";

use crate::dfx::account_identifier::AccountIdentifier;
use crate::dfx::icpts::{ICPTs, TRANSACTION_FEE};
use crate::dfx::ledger_types::{AccountIdBlob, Memo, TimeStamp, TransferArgs, TransferResult};

#[derive(CandidType)]
pub struct AccountBalanceArgs {
    pub account: String,
}

const TRANSFER_METHOD: &str = "transfer";

pub async fn transfer(agent: &Agent, amount: ICPTs, to: AccountIdBlob) -> Result<u64, String> {
    let timestamp_nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    let memo = Memo(0);
    let fee = TRANSACTION_FEE;
    // let canister_id = MAINNET_LEDGER_CANISTER_ID;

    let canister = Canister::builder()
        .with_agent(&agent)
        .with_canister_id(MAINNET_LEDGER_CANISTER_ID)
        .build()
        .unwrap();

    let waiter = garcon::Delay::builder()
        .throttle(std::time::Duration::from_millis(500))
        .timeout(std::time::Duration::from_secs(60 * 5))
        .build();

    let (response,) = canister
        .update_(TRANSFER_METHOD)
        .with_arg_raw(
            Encode!(&TransferArgs {
                memo,
                amount,
                fee,
                from_subaccount: None,
                to,
                created_at_time: Some(TimeStamp { timestamp_nanos })
            })
            .unwrap(),
        )
        .build::<(TransferResult,)>()
        .call_and_wait(waiter)
        .await
        .unwrap();

    println!("{:?}", response);

    match response {
        Ok(r) => Ok(r),
        Err(e) => Err(format!("{}", e)),
    }
}

pub async fn get_balance_q_agent(acc_id: AccountIdentifier, agent: &Agent) -> String {
    let canister = Canister::builder()
        .with_agent(&agent)
        .with_canister_id(MAINNET_LEDGER_CANISTER_ID)
        .build()
        .unwrap();

    let (response,) = canister
        .query_(ACCOUNT_BALANCE_METHOD)
        .with_arg_raw(
            Encode!(&AccountBalanceArgs {
                account: acc_id.to_string()
            })
            .unwrap(),
        )
        .build::<(ICPTs,)>()
        .call()
        .await
        .unwrap();

    println!("{}", response);

    format!("{}", response)
}
