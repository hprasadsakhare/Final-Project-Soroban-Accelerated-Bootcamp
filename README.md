**# Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:
```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.


**# Lost and Found Registry 

### Introduction

The Lost and Found Registry is a smart contract designed to facilitate the registration and claiming of lost items on a blockchain platform. This contract allows users to register found items, claim ownership of those items, and retrieve information about registered items.

### Features

Register Found Item: Users can register found items by providing a unique item ID and a description of the item. Once registered, the item details are stored on the blockchain.

Claim Item: If a user believes they are the rightful owner of a registered item, they can claim ownership by providing the item ID. Upon successful verification, ownership of the item is transferred to the claimant.

Get Item Details: Users can retrieve details about a registered item by providing its item ID.

### Usage
Register Found Item
To register a found item, call the register_found_item function with the following parameters:

from: The address of the user registering the item.
item_id: A unique identifier for the item.
description: A description of the found item.

### Claim Item
To claim ownership of a registered item, call the claim_item function with the following parameters:

from: The address of the user claiming ownership.
item_id: The unique identifier of the item to be claimed.

### Get Item Details

To retrieve details about a registered item, call the get_item_details function with the following parameter:

item_id: The unique identifier of the item to retrieve details for.
