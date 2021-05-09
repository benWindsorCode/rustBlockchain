use std::time::SystemTime;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Hash, Debug)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u32
}

#[derive(Hash, Debug)]
pub struct Block {
    pub index: u32,
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
    fn new(sender: String, recipient: String, amount: u32) -> Transaction {
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
        let first_hash = "".to_string();
        let first_proof = 100;
        let first_block = Block::new(first_hash, first_proof);
        Blockchain { chain: vec![first_block], current_transactions: Vec::new() }
    }

    fn new_block(mut self) {

    }

    // Adds transaction, returns index of new block that will hold this transaction
    fn new_transaction(mut self, sender: String, recipient: String, amount: u32) -> usize {
        self.current_transactions.push(Transaction{ sender, recipient, amount});

        return self.last_block_index() + 1;
    }

    // Tutorial returns the last block itself, however I couldn't get the copying to work given Vec<Transaction> in block so just return index
    fn last_block_index(self) -> usize {
        self.chain.len()
    }

    // How do I make this static? is it just by leaving self out?
    fn hash(block: Block) -> String {
        println!("Hashing");
        let mut s = DefaultHasher::new();
        block.hash(&mut s);
        s.finish().to_string()
    }

    pub fn proof_of_work(self, last_proof: u64) -> u64 {
        let mut proof = 0;
        while Blockchain::valid_proof(last_proof, proof) == false {
            proof += 1;
        }

        proof
    }

    fn valid_proof(last_proof: u64, proof: u64) -> u64 {
        println!("Proof of work");
        let mut s = DefaultHasher::new();
        let test_val = last_proof + proof;
        test_val.hash(&mut s);
        let output = s.finish();
        output
    }
}