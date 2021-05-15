use std::time::SystemTime;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fmt;

const GENESIS_ACCOUNT: &str = "Genesis";

/// A single transaction between a sender and recipient
#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64
}

/// A block on the chain, storing a vector of transactions and proof of work
#[derive(Debug)]
pub struct Block {
    pub index: usize,
    pub timestamp: SystemTime,
    pub transactions: Vec<Transaction>,
    pub proof: u64,
    pub previous_hash: String
}

/// A blockchain is made up of a vector of blocks, and a vector of current pending transactions
#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub current_transactions: Vec<Transaction>
}

impl Transaction {
    fn new(sender: String, recipient: String, amount: f64) -> Transaction {
        Transaction { sender, recipient, amount }
    }
}

impl Block {
    fn new(previous_hash: String, proof: u64) -> Block {
        Block { index: 0, timestamp: SystemTime::now(), transactions: Vec::new(), proof: proof, previous_hash: previous_hash }
    }
}

impl Blockchain {
    /// Create a new chain with two sender accounts credited 5000 of the token
    pub fn new() -> Blockchain {
        let first_hash = "".to_string();
        let first_proof = 100;
        let mut first_block = Block::new(first_hash, first_proof);
        let genesis_transaction_1 = Transaction::new(GENESIS_ACCOUNT.to_string(), "senderA".to_string(), 5000.);
        let genesis_transaction_2 = Transaction::new(GENESIS_ACCOUNT.to_string(), "senderC".to_string(), 5000.);
        first_block.transactions = vec![genesis_transaction_1, genesis_transaction_2];
        Blockchain { chain: vec![first_block], current_transactions: Vec::new() }
    }

    /// Add a new block to the end of the chain as long as proof of work provided is valid
    pub fn new_block(&mut self, proof: u64) {
        // Borrow last block in order to hash
        let last_block = &self.chain[self.last_block_index()];

        if !Blockchain::valid_proof(last_block.proof, proof) {
            panic!("Invalid proof ({}) found. Couldn't add block to blockchain.", proof);
        } 

        let block = Block {
            index: self.last_block_index() + 1,
            timestamp: SystemTime::now(),
            transactions: self.current_transactions.clone(),
            proof: proof,
            previous_hash: Blockchain::hash(&last_block)
        };

        self.current_transactions = Vec::new();

        self.chain.push(block);
    }

    /// Scan through transactions and return hash map of HashMap<Account name, Balance in account>
    pub fn balances(&self) -> HashMap<&String, f64> {
        let mut balances: HashMap<&String, f64> = HashMap::new();
        
        // For each transaction, deduct it from sender and add it to reciever account
        for block in &self.chain {
            for transaction in &block.transactions {
                let sender_bal = balances.get(&transaction.sender).unwrap_or(&0.).clone();
                let recipient_bal = balances.get(&transaction.recipient).unwrap_or(&0.).clone();
                let amount = transaction.amount;

                balances.insert(&transaction.sender, sender_bal - amount);
                balances.insert(&transaction.recipient, recipient_bal + amount);
            }
        }

        balances
    }

    /// Create new coins for the recipient from the genesis account
    pub fn create_coin(&mut self, recipient: String, amount: f64) -> usize {
        self.new_transaction(GENESIS_ACCOUNT.to_string(), recipient, amount)
    }

    /// Adds a transaction, returns index of new block that will hold this transaction
    pub fn new_transaction(&mut self, sender: String, recipient: String, amount: f64) -> usize {
        let current_balances = self.balances();
        let sender_bal = current_balances.get(&sender).unwrap_or(&0.);

        // Ensure sender has enough balance to send amount. Only allow negative balances from the Genesis account.
        if GENESIS_ACCOUNT != sender && sender_bal - amount < 0. {
            panic!("Sender bal {} - amount {} < 0, cannot complete transaction", sender_bal, amount);
        }

        let new_transaction = Transaction::new(sender, recipient, amount);

        self.current_transactions.push(new_transaction);

        return self.last_block_index() + 1;
    }

    /// Return index of the last block in the chain
    fn last_block_index(&self) -> usize {
        self.chain.len() - 1
    }

    /// Simple hash of a block
    fn hash(block: &Block) -> String {
    // TODO: make better than just hashing the timestamp
        let mut s = DefaultHasher::new();
        block.timestamp.hash(&mut s);
        s.finish().to_string()
    }

    /// Fetch the last proof in the chain
    pub fn last_proof(&self) -> u64 {
        let last_block = &self.chain[self.last_block_index()];

        last_block.proof
    }

    /// Perform proof of work until a valid proof produced
    pub fn proof_of_work(last_proof: u64) -> u64 {
        let mut proof = 0;
        while Blockchain::valid_proof(last_proof, proof) == false {
            proof += 1;
        }

        proof
    }

    /// Very simple proof function using modular arithmetic, easy to compute and check
    fn valid_proof(last_proof: u64, proof: u64) -> bool {
        let mut s = DefaultHasher::new();
        let test_val = last_proof + proof;
        test_val.hash(&mut s);
        let new_hash = s.finish();
    
        new_hash % 23 == 0
    }
}

impl fmt::Display for Blockchain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut blockchain_for_display: String = String::new();

        // String together all of the blocks
        for block in &self.chain {
            let block_string = format!("{}", block);
            blockchain_for_display.push_str(&block_string);

            // Arrow between blocks
            blockchain_for_display.push_str("\n|\nv\n");
        }

        let current_transactions = format!("Current transactions pending: {}\n", self.current_transactions.len());
        blockchain_for_display.push_str(&current_transactions);
        for transaction in &self.current_transactions {
            let transaction_string = format!("{}\n", transaction);
            blockchain_for_display.push_str(&transaction_string);
        }

        write!(f, "{}", blockchain_for_display)
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut block_for_display: String = format!(
            "------ Block: {} ------\n\
            | Time: {:?}\n\
            | Proof: {}\n\
            | Current Hash: {}\n\
            | Previous Hash: {}\n\
            | #Transactions: {}\n",
            self.index, 
            self.timestamp,
            self.proof,
            Blockchain::hash(self),
            self.previous_hash,
            self.transactions.len()
        );

        let mut transactions = String::new();
        for transaction in &self.transactions {
            let transaction_string = format!("|-| {}\n", transaction);
            transactions.push_str(&transaction_string);
        }

        block_for_display.push_str(&transactions);

        let end_line = "----------------------".to_string();
        block_for_display.push_str(&end_line);

        write!(f, "{}", block_for_display)
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let transaction_for_display: String = format!(
            "Transaction for {}: {} (sender) -> {} (recipient)",
            self.amount,
            self.sender,
            self.recipient
        );

        write!(f, "{}", transaction_for_display)
    }
}