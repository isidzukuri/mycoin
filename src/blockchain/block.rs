use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: usize,
    pub timestamp: u64,
    pub proof: u64,
    pub previous_hash: String,
    pub transactions: Vec<Transaction>,
}
