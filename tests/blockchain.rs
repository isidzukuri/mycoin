use mycoin::blockchain::blockchain::BlockChain;

#[test]
fn new_test() {
    let obj = BlockChain::new();

    assert_eq!(obj.chain.len(), 1);
    assert_eq!(obj.chain[0].previous_hash, "0");
    assert_eq!(obj.chain[0].index, 0);
    assert_eq!(obj.chain[0].proof, 1);
}

#[test]
fn get_last_block_test() {
    let blockchain = BlockChain::new();
    let obj = blockchain.get_last_block().unwrap();

    assert_eq!(obj.previous_hash, "0");
    assert_eq!(obj.index, 0);
    assert_eq!(obj.proof, 1);
}

#[test]
fn proof_of_work_test() {
    let blockchain = BlockChain::new();
    let result = blockchain.proof_of_work(2);

    assert_eq!(result, 95694);
}

#[test]
fn hash_test() {
    let blockchain = BlockChain::new();
    let result = blockchain.hash(blockchain.chain.first().unwrap());

    assert_eq!(result.len(), 64);
}

#[test]
fn is_chain_valid_test() {
    let mut blockchain = BlockChain::new();

    let new_proof = blockchain.proof_of_work(blockchain.chain.first().unwrap().proof);
    let previous_hash = blockchain.hash(blockchain.chain.first().unwrap());
    blockchain.create_block(new_proof, previous_hash);

    assert_eq!(blockchain.is_valid(&blockchain.chain), true);
}

#[test]
fn is_chain_valid_invalid_prev_hash_test() {
    let mut blockchain = BlockChain::new();

    let new_proof = blockchain.proof_of_work(blockchain.chain.first().unwrap().proof);
    let previous_hash = blockchain.hash(blockchain.chain.first().unwrap());
    blockchain.create_block(new_proof, previous_hash);
    blockchain.create_block(new_proof, "random string".to_string());

    assert_eq!(blockchain.is_valid(&blockchain.chain), false);
}

#[test]
fn mine_block_test() {
    let mut blockchain = BlockChain::new();

    let new_block = blockchain.mine_block();

    assert_eq!(new_block.previous_hash.len(), 64);
    assert_eq!(new_block.index, 1);
    assert_eq!(new_block.proof, 533)
}

#[test]
fn add_node_test() {
    let mut blockchain = BlockChain::new();

    blockchain.add_node("http://127.0.0.2:5000".to_string());
    blockchain.add_node("http://127.0.0.2:5000".to_string());
    blockchain.add_node("http://127.0.0.2:5001".to_string());
    blockchain.add_node("http://127.0.0.1".to_string());

    assert_eq!(blockchain.nodes.len(), 3);
    assert_eq!(blockchain.nodes[0], "127.0.0.2:5000");
    assert_eq!(blockchain.nodes[1], "127.0.0.2:5001");
    assert_eq!(blockchain.nodes[2], "127.0.0.1");
}

// #[test]
// fn add_transaction_test() {}

// #[test]
// fn replace_chain_test() {}
