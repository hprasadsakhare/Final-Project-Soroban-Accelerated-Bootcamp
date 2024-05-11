#![no_std]

use soroban_sdk::{ contract, contractimpl, contracttype, Address, Env, String };

#[derive(Clone)]
#[contracttype]
pub enum StorageKey {
    FoundItems(u64), // Key for storing found items
}

#[derive(Clone)]
#[contracttype]
pub struct ItemDetails {
    owner: Address,
    description: String,
    claimed: bool,
}

#[contract]
pub struct LostAndFoundRegistry;

#[contractimpl]
impl LostAndFoundRegistry {
    pub fn register_found_item(env: Env, from: Address, item_id: u64, description: String) {
        from.require_auth();
        // Check if the item is already registere
        let itemid: u32 = env
            .storage()
            .instance()
            .get(&StorageKey::FoundItems(item_id))
            .unwrap_or(0);
        if itemid != 0 {
            panic!("Item already exists");
        }
        // Store the item details in storage
        let item_details = ItemDetails {
            owner: from,
            description,
            claimed: false,
        };
        env.storage().instance().set(&StorageKey::FoundItems(item_id), &item_details);
    }

    pub fn claim_item(env: Env, item_id: u64, from: Address) {
        from.require_auth();
        // Get item details from storage
        let mut item_details: ItemDetails = env
            .storage()
            .instance()
            .get(&StorageKey::FoundItems(item_id))
            .unwrap();

        // Ensure the item exists and is not already claimed
        if item_details.claimed {
            panic!("Item already claimed");
        }

        // Transfer ownership to the claimant
        item_details.owner = from;
        item_details.claimed = true;

        // Update item details in storage
        env.storage().instance().set(&StorageKey::FoundItems(item_id), &item_details);

        // Optionally, perform additional actions such as transferring funds or emitting events
    }

    pub fn get_item_details(env: Env, item_id: u64) -> Option<ItemDetails> {
        env.storage().instance().get(&StorageKey::FoundItems(item_id))
    }
}

mod test;
