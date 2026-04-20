#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String};

// Melakukan test disini
#[test]
fn test_add_item() {
    let env = Env::default();
    let contract_id = env.register_contract(None, InventoryContract);
    let client = InventoryContractClient::new(&env, &contract_id);

    let user = Address::random(&env);

    client.add_item(&user, &"Laptop".into(), &10, &5000);
}