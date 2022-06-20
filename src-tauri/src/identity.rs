use ic_agent::{
    agent::http_transport::ReqwestHttpReplicaV2Transport, identity::BasicIdentity, Agent, Identity,
};

use crate::call_helpers::DEFAULT_IC_GATEWAY;

pub fn use_identity(pem_path: &str) -> Result<impl Identity, String> {
    // let pem_path = "../identity2.pem".to_string();

    match BasicIdentity::from_pem_file(pem_path) {
        Ok(id) => Ok(id),
        Err(_) => Err("Could not read identity file".into()),
    }
}

pub fn agent_from_pem_file(pem_path: &str) -> Result<Agent, String> {
    let timeout = std::time::Duration::from_secs(60 * 5);

    let transport = ReqwestHttpReplicaV2Transport::create(DEFAULT_IC_GATEWAY)
        .expect("Failed to create Reqwest transport");

    let my_identity = use_identity(pem_path)?;

    let agent = Agent::builder()
        .with_transport(transport)
        .with_identity(my_identity)
        .with_ingress_expiry(Some(timeout))
        .build()
        .expect("Failed to build agent");

    Ok(agent)
}

// pub fn test_default_identity() {
//     let my_identity = use_identity("/home/devel/.config/dfx/identity/default/identity.pem");

//     let my_principal = my_identity.sender().unwrap();
//     println!("Principal: {}", my_principal.to_text());

//     let my_account_identifier = AccountIdentifier::new(my_principal, Some(SUB_ACCOUNT_ZERO));

//     println!("Account Identifier: {}", &my_account_identifier);
// }
