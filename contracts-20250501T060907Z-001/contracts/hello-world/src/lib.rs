#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec, log};

#[derive(Clone)]
#[contracttype]
pub struct Request {
    requester: Address,
    description: String,
    status: String, // "Open", "InProgress", "Resolved"
    timestamp: u64,
}

#[contracttype]
pub enum DataKey {
    Requests,
    Owner,
}

#[contract]
pub struct MaintenanceLogger;

#[contractimpl]
impl MaintenanceLogger {
    pub fn init(env: Env, owner: Address) {
        owner.require_auth();
        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage().instance().set(&DataKey::Requests, &Vec::<Request>::new(&env));
    }



    pub fn update_status(env: Env, index: u32, new_status: String) {
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth();

        let mut requests: Vec<Request> = env.storage().instance().get(&DataKey::Requests).unwrap();
        if let Some(mut req) = requests.get(index) {
            req.status = new_status;
            requests.set(index, req);
            env.storage().instance().set(&DataKey::Requests, &requests);
        } else {
            panic!("Request not found");
        }
    }

    pub fn get_all_requests(env: Env) -> Vec<Request> {
        env.storage().instance().get(&DataKey::Requests).unwrap()
    }
}
