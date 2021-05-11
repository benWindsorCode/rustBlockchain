use blockchain::blockchain::Blockchain;

fn main() {
    let mut chain = Blockchain::new();

    println!("(1) INITIAL BLOCKCHAIN");
    println!("{}", chain);
    chain.new_transaction("senderA".to_string(), "recipientA".to_string(), 5.);
    chain.new_transaction("senderC".to_string(), "recipientC".to_string(), 12.);

    println!("(2) BLOCKCHAIN WITH TRANSACTIONS");
    println!("{}", chain);
    let last_proof = chain.last_proof();
    let new_proof = Blockchain::proof_of_work(last_proof);

    // Hypothetical miner gets one coin for completing the new proof
    chain.create_coin("minerA".to_string(), 1.);
    chain.new_block(new_proof);

    println!("(3) BLOCKCHAIN WITH NEW BLOCKS");
    chain.new_transaction("senderA".to_string(), "recipientA".to_string(), 32.);
    let last_proof = chain.last_proof();
    let new_proof = Blockchain::proof_of_work(last_proof);

    // Hypothetical miner gets one coin for completing the new proof
    chain.create_coin("minerB".to_string(), 0.8);
    chain.new_block(new_proof);
    println!("{}", chain);

    println!("(4) BALANCES AT END");
    println!("{:?}", chain.balances());
}
