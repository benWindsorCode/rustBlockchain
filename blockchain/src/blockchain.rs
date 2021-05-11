use std::time::SystemTime;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64
}

#[derive(Debug)]
pub struct Block {
    pub index: usize,
    pub timestamp: SystemTime,
    pub transactions: Vec<Transaction>,
    pub proof: u64,
    pub previous_hash: String
}

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
    pub fn new() -> Blockchain {
        // TODO: make this a const/property of the Blockchain
        // Genesis account name
        let genesis_account = "GENESIS".to_string();

        // println!("Creating new blockchain with genesis block");
        let first_hash = "".to_string();
        let first_proof = 100;
        let mut first_block = Block::new(first_hash, first_proof);
        let genesis_transaction_1 = Transaction::new(genesis_account.clone(), "senderA".to_string(), 5000.);
        let genesis_transaction_2 = Transaction::new(genesis_account.clone(), "senderC".to_string(), 5000.);
        first_block.transactions = vec![genesis_transaction_1, genesis_transaction_2];
        Blockchain { chain: vec![first_block], current_transactions: Vec::new() }
    }

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

        // println!("Adding new block: {:?}", block);
        self.chain.push(block);
    }

    // Scans through transactions and returns balances
    pub fn balances(&self) -> HashMap<&String, f64> {
        let mut balances: HashMap<&String, f64> = HashMap::new();
        
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

    // Create some new coin for the recipient
    pub fn create_coin(&mut self, recipient: String, amount: f64) -> usize {
        // TODO: make this a const/property of the Blockchain
        // Genesis account name
        let genesis_account = "GENESIS".to_string();

        self.new_transaction(genesis_account, recipient, amount)
    }

    // Adds transaction, returns index of new block that will hold this transaction
    pub fn new_transaction(&mut self, sender: String, recipient: String, amount: f64) -> usize {
        // TODO: deal with genesis account here, should be allowed negative balance
        let current_balances = self.balances();
        let sender_bal = current_balances.get(&sender).unwrap_or(&0.);
        if sender_bal - amount < 0. {
            panic!("Sender bal {} - amount {} < 0, cannot complete transaction", sender_bal, amount);
        }

        let new_transaction = Transaction::new(sender, recipient, amount);

        // println!("Adding new transaction: {:?}", new_transaction);
        self.current_transactions.push(new_transaction);

        return self.last_block_index() + 1;
    }

    // Tutorial returns the last block itself, however I couldn't get the copying to work given Vec<Transaction> in block so just return index
    fn last_block_index(&self) -> usize {
        self.chain.len() - 1
    }

    // How do I make this static? is it just by leaving self out?
    // TODO: make better than just hashing the timestamp
    fn hash(block: &Block) -> String {
        // println!("Hashing");
        let mut s = DefaultHasher::new();
        block.timestamp.hash(&mut s);
        s.finish().to_string()
    }

    pub fn last_proof(&self) -> u64 {
        let last_block = &self.chain[self.last_block_index()];

        last_block.proof
    }

    pub fn proof_of_work(last_proof: u64) -> u64 {
        let mut proof = 0;
        while Blockchain::valid_proof(last_proof, proof) == false {
            proof += 1;
        }

        proof
    }

    // TODO: finish this, for now we just check mod ten. The example uses the last four digits are zero
    fn valid_proof(last_proof: u64, proof: u64) -> bool {
        // println!("Proof of work for proof: {}", proof);
        let mut s = DefaultHasher::new();
        let test_val = last_proof + proof;
        test_val.hash(&mut s);
        let new_hash = s.finish();
        // let digits = new_hash.to_string().chars();
    
        // Check if multiple of twenty three
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