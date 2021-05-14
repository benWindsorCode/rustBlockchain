# rustBlockchain
A simple implementation of a blockchain in rust, with a cursive UI to go with it for easy interaction.

# Installation
It is a simple project to setup:
1. Clone the repo
2. Install rust and cargo https://rustup.rs/
2. Install ncurses per https://github.com/gyscos/cursive/wiki/Install-ncurses

# Running
From within the repo directory:
1. Enter the cargo project directory
```
cd blockchain
```
2. Build the project
```
cargo build
```
3. Run the project
cargo run

# Usage
From within the UI press escape to access the menu.

You can do the following from the 'Actions' menu:
- View the chain with 'View Chain'
- See which accounts/wallets on the chain have what balances with 'Show Balances'
- Create a new transaction with 'New Transaction'. This will show up on the 'View Chain' as a current transaction until immortalised into a block
- Simulate the act of mining and block creation with 'Create Block', this takes current transactions and forms a new block, crediting some of the token to a mining account

To quit select 'Quit' from the menu bar or use Ctrl+C.

Note: there is no persistance between runs, this is just a simple local toy blockchain.

# Code Tour
The project is laid out as follows:
- main.rs has the entry point 
- blockchain.rs
- interface.rs

# Feature Wishlist
To flesh this out further the following would be required:
- Persistance of the chain, saving and loading to disk
- Improvement of the 'mining' both in terms of proof of work and allocating of token reward
- Creation of multiple nodes and a concensus algorithm between nodes
- Fleshing out with a REST API to allow it to run on a simple server