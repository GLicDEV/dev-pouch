#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod call_helpers;
mod dfx;
mod identity;
mod utils;

use std::{collections::HashMap, str::FromStr, sync::Mutex};

use call_helpers::get_balance_q_agent;
use candid::Principal;
use dfx::account_identifier::{AccountIdentifier, SUB_ACCOUNT_ZERO};
use ic_agent::{Agent, Identity};
use identity::{agent_from_pem_file, use_identity};
use serde::Serialize;
use tauri::command;

use crate::{call_helpers::transfer, dfx::icpts::ICPTs};

// pub struct AppState(pub RwLock<InnerState>);
pub struct AppState(pub Mutex<InnerState>);

#[derive(Debug, Default)]
pub struct InnerState {
    identities: HashMap<String, DfxIdentity>,
}

#[derive(Debug, Clone)]
pub struct DfxIdentity {
    principal: Principal,
    agent: Agent,
}

impl DfxIdentity {
    pub fn get_agent(&self) -> Option<&Agent> {
        Some(&self.agent)
    }
}

#[command]
async fn set_dfx_identity_path(
    state: tauri::State<'_, AppState>,
    argument: Option<String>,
) -> Result<String, String> {
    match argument {
        Some(arg) => {
            if arg.is_empty() {
                return Err("Must provide a valid path".into());
            }
            let my_identity = use_identity(&arg)?;
            let my_agent = agent_from_pem_file(&arg)?;
            let my_principal = my_identity.sender().unwrap();

            let mut state_guard = state.0.lock().unwrap();

            state_guard.identities.insert(
                "default".to_string(),
                DfxIdentity {
                    principal: my_principal,
                    agent: my_agent,
                },
            );
        }
        None => return Err("Must provide a valid path".into()),
    };

    Ok("".into())
}

#[command]
async fn cmd_get_balance_agent(
    state: tauri::State<'_, AppState>,
    _argument: Option<String>,
) -> Result<String, String> {
    let state_guard = state.0.lock().unwrap().identities.get("default").cloned();

    match state_guard {
        Some(sg) => {
            let agent = &sg.agent;

            let my_principal = sg.principal;

            let my_account_identifier =
                AccountIdentifier::new(my_principal, Some(SUB_ACCOUNT_ZERO));

            let balance = get_balance_q_agent(my_account_identifier, agent).await;

            Ok(balance)
        }
        None => Err("No identity set yet".into()),
    }
}

#[derive(Serialize)]
struct RefreshResponse {
    principal: Principal,
    account_identifier: AccountIdentifier,
    balance: String,
}

#[command]
async fn cmd_refresh_wallet(
    state: tauri::State<'_, AppState>,
    _argument: Option<String>,
) -> Result<RefreshResponse, String> {
    let state_guard = state.0.lock().unwrap().identities.get("default").cloned();

    match state_guard {
        Some(sg) => {
            let agent = &sg.agent;

            let my_principal = sg.principal;

            let my_account_identifier =
                AccountIdentifier::new(my_principal, Some(SUB_ACCOUNT_ZERO));

            let balance = get_balance_q_agent(my_account_identifier, agent).await;

            Ok(RefreshResponse {
                principal: my_principal,
                account_identifier: my_account_identifier,
                balance,
            })
        }
        None => Err("No identity set yet".into()),
    }
}

#[command]
async fn cmd_send_funds(
    state: tauri::State<'_, AppState>,
    send_amount: Option<String>,
    send_to: Option<String>,
) -> Result<u64, String> {
    let state_guard = state.0.lock().unwrap().identities.get("default").cloned();

    match state_guard {
        Some(sg) => {
            if let None = send_amount {
                return Err("No amount set".into());
            }

            if let None = send_to {
                return Err("No destination address set".into());
            }

            let send_amount = send_amount.unwrap();
            let send_to = send_to.unwrap();

            let amount = ICPTs::from_str(&send_amount)?;
            let to = AccountIdentifier::from_str(&send_to)?.to_address();

            let agent = &sg.agent;

            // let my_principal = sg.principal;

            // let my_account_identifier =
            //     AccountIdentifier::new(my_principal, Some(SUB_ACCOUNT_ZERO));

            println!("Sending {} ICP to {}", send_amount, send_to);

            transfer(agent, amount, to).await
        }
        None => Err("No identity set yet".into()),
    }
}

// #[command]
// fn cmd(argument: String) -> Result<String, String> {
//     if argument.len() > 0 {
//         Ok(format!("{}", argument))
//     } else {
//         Err(String::from("Invalid argument"))
//     }
// }

fn main() {
    tauri::Builder::default()
        .manage(AppState(Mutex::new(InnerState::default())))
        .invoke_handler(tauri::generate_handler![
            set_dfx_identity_path,
            cmd_get_balance_agent,
            cmd_refresh_wallet,
            cmd_send_funds
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
