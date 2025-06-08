use chrono::Utc;
use sha2::{Sha256, Digest};
use std::time::Instant;
use std::io::{self, Write};

#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: i64,
    data: String,
    previous_hash: String,
    nonce: u64,
    hash: String,
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String) -> Self {
        let mut block = Block {
            index,
            timestamp: Utc::now().timestamp(),
            data,
            previous_hash,
            nonce: 0,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let block_string = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        );
        hasher.update(block_string.as_bytes());
        hex::encode(hasher.finalize())
    }

    fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        let start_time = Instant::now();
        let mut attempts = 0;

        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
            attempts += 1;

            // Print progress every 100,000 attempts
            if attempts % 100_000 == 0 {
                print!("\rAttempts: {}", attempts);
                io::stdout().flush().unwrap();
            }
        }

        let duration = start_time.elapsed();
        println!("\n\nBlock Mined!");
        println!("Hash: {}", self.hash);
        println!("Nonce: {}", self.nonce);
        println!("Attempts: {}", attempts);
        println!("Time taken: {:.2} seconds", duration.as_secs_f64());
    }
}

fn main() {
    println!("Welcome to Nonce Mining Simulation!");
    
    // Get difficulty from user
    print!("Enter mining difficulty (number of leading zeros): ");
    io::stdout().flush().unwrap();
    
    let mut difficulty = String::new();
    io::stdin()
        .read_line(&mut difficulty)
        .expect("Failed to read difficulty");
    
    let difficulty: usize = difficulty.trim().parse().expect("Please enter a valid number");

    // Create and mine a block
    let mut block = Block::new(
        1,
        "Genesis Block".to_string(),
        "0".repeat(64),
    );

    println!("\nStarting mining with difficulty {}...", difficulty);
    block.mine_block(difficulty);
}
