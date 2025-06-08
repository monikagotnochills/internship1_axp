use chrono::Utc;
use sha2::{Sha256, Digest};
use std::fmt;
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Block {
    index: u32,
    timestamp: i64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u32,
}

impl Block {
    fn new(index: u32, data: String, previous_hash: String) -> Block {
        let timestamp = Utc::now().timestamp();
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let input = format!("{}{}{}{}{}", self.index, self.timestamp, self.data, self.previous_hash, self.nonce);
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn mine_block(&mut self, difficulty: u32) {
        let target = "0".repeat(difficulty as usize);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Block #{}: Timestamp: {}, Data: {}, Previous Hash: {}, Hash: {}, Nonce: {}", 
            self.index, self.timestamp, self.data, self.previous_hash, self.hash, self.nonce)
    }
}

fn main() {
    let mut blockchain: Vec<Block> = Vec::new();

    // Create the first block (genesis block)
    let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
    blockchain.push(genesis_block);

    // Create the second block
    let second_block = Block::new(1, "Second Block".to_string(), blockchain[0].hash.clone());
    blockchain.push(second_block);

    // Create the third block
    let third_block = Block::new(2, "Third Block".to_string(), blockchain[1].hash.clone());
    blockchain.push(third_block);

    // Display all blocks
    for block in &blockchain {
        println!("{}", block);
    }

    // Allow user to modify Block 1
    println!("\nEnter new data for Block 1:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    blockchain[1].data = input.trim().to_string();
    blockchain[1].hash = blockchain[1].calculate_hash();

    // Display all blocks after modification
    println!("\nAfter modifying Block 1:");
    for block in &blockchain {
        println!("{}", block);
    }
}
