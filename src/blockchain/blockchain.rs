use crate::blockchain::block::Block;
use crate::transaction::Transaction;
use reqwest;
use serde_json;
use sha256::digest;
use std::time::SystemTime;
use url::Url;

#[derive(Debug)]
pub struct BlockChain {
    pub chain: Vec<Block>,
    pub transactions: Vec<Transaction>,
    pub nodes: Vec<String>,
}

impl BlockChain {
    pub fn new() -> Self {
        let mut obj = Self {
            chain: vec![],
            transactions: vec![],
            nodes: vec![],
        };

        obj.create_block(1, "0".to_string());

        obj
    }

    pub fn create_block(&mut self, proof: u64, previous_hash: String) -> &Block {
        let block = Block {
            index: self.chain.len(),
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            proof: proof,
            previous_hash: previous_hash,
            transactions: self.transactions.clone(),
        };

        self.chain.push(block);
        self.transactions = vec![];
        self.get_last_block().unwrap()
    }

    pub fn get_last_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    pub fn proof_of_work(&self, previous_proof: u64) -> u64 {
        let mut new_proof: u64 = 1;
        let mut check_proof = false;

        while check_proof == false {
            let proof_exists = self
                .chain
                .iter()
                .filter(|item| item.proof == new_proof)
                .collect::<Vec<_>>()
                .len()
                > 0;

            if !proof_exists && Self::hashed_proof(previous_proof, new_proof).starts_with("0000") {
                check_proof = true;
            } else {
                new_proof += 1;
            }
        }

        new_proof
    }

    fn hashed_proof(previous_proof: u64, new_proof: u64) -> String {
        let input = (new_proof.pow(2).abs_diff(previous_proof.pow(2))).to_string();
        digest(input)
    }

    pub fn hash(&self, block: &Block) -> String {
        digest(serde_json::to_string(block).unwrap())
    }

    pub fn is_valid(&self, chain: &Vec<Block>) -> bool {
        let mut previous_block = &chain[0];
        let mut block_index = 1;

        while block_index < chain.len() {
            let block = &chain[block_index];
            if block.previous_hash != self.hash(&previous_block) {
                return false;
            }

            if !Self::hashed_proof(previous_block.proof, block.proof).starts_with("0000") {
                return false;
            }

            previous_block = block;
            block_index += 1;
        }

        true
    }

    pub fn mine_block(&mut self) -> &Block {
        let previous_block = self.get_last_block().unwrap();

        let proof = self.proof_of_work(previous_block.proof);
        let previous_hash = self.hash(previous_block);

        self.create_block(proof, previous_hash)
    }

    pub fn add_transaction(&mut self, sender: String, receiver: String, amount: f64) -> usize {
        let transaction = Transaction {
            sender: sender,
            receiver: receiver,
            amount: amount,
        };

        self.transactions.push(transaction);

        self.get_last_block().unwrap().index + 1
    }

    pub fn add_node(&mut self, url: String) {
        let parsed_url = Url::parse(&url).expect("Invalid node url");
        let host = parsed_url.host_str().unwrap().to_string();
        let port = match parsed_url.port() {
            Some(val) => format!(":{}", val.to_string()),
            _ => "".to_string(),
        };

        let address = format!("{}{}", host, port);

        if self
            .nodes
            .iter()
            .position(|each| *each == address)
            .is_none()
        {
            self.nodes.push(address);
        }
    }

    pub async fn replace_chain(&mut self) -> bool {
        let mut longest_chain: Option<Vec<Block>> = None;
        let mut max_length = self.chain.len();

        for node in self.nodes.iter() {
            let response = reqwest::get(format!("http://{}/chain", node))
                .await
                .expect("Node request failed");

            if response.status() == 200 {
                let parsed_body = response.json::<serde_json::Value>().await.unwrap();

                if serde_json::from_value::<usize>(parsed_body["length"].clone()).unwrap()
                    > max_length
                {
                    let received_chain: Vec<Block> =
                        serde_json::from_value(parsed_body["chain"].clone()).unwrap();
                    if self.is_valid(&received_chain) {
                        max_length = received_chain.len();
                        longest_chain = Some(received_chain);
                    }
                }
            }
        }

        if longest_chain.is_some() {
            self.chain = longest_chain.unwrap();
            return true;
        } else {
            return false;
        }
    }
}
