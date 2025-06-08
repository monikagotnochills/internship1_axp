use rand::Rng;
use std::collections::HashMap;

// Validator structures for different consensus mechanisms
#[derive(Debug)]
struct PowValidator {
    id: String,
    power: u64,
}

#[derive(Debug)]
struct PosValidator {
    id: String,
    stake: u64,
}

#[derive(Debug)]
struct DposValidator {
    id: String,
    votes: u32,
}

// Consensus mechanism traits
trait Consensus {
    fn select_validator(&self) -> String;
    fn explain_selection(&self) -> String;
}

// Proof of Work implementation
struct PowConsensus {
    validators: Vec<PowValidator>,
}

impl PowConsensus {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        PowConsensus {
            validators: vec![
                PowValidator {
                    id: "Miner1".to_string(),
                    power: rng.gen_range(1..100),
                },
                PowValidator {
                    id: "Miner2".to_string(),
                    power: rng.gen_range(1..100),
                },
                PowValidator {
                    id: "Miner3".to_string(),
                    power: rng.gen_range(1..100),
                },
            ],
        }
    }
}

impl Consensus for PowConsensus {
    fn select_validator(&self) -> String {
        self.validators
            .iter()
            .max_by_key(|v| v.power)
            .unwrap()
            .id
            .clone()
    }

    fn explain_selection(&self) -> String {
        format!(
            "PoW: Selected validator with highest computational power.\nValidators: {:?}",
            self.validators
        )
    }
}

// Proof of Stake implementation
struct PosConsensus {
    validators: Vec<PosValidator>,
}

impl PosConsensus {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        PosConsensus {
            validators: vec![
                PosValidator {
                    id: "Staker1".to_string(),
                    stake: rng.gen_range(100..1000),
                },
                PosValidator {
                    id: "Staker2".to_string(),
                    stake: rng.gen_range(100..1000),
                },
                PosValidator {
                    id: "Staker3".to_string(),
                    stake: rng.gen_range(100..1000),
                },
            ],
        }
    }
}

impl Consensus for PosConsensus {
    fn select_validator(&self) -> String {
        self.validators
            .iter()
            .max_by_key(|v| v.stake)
            .unwrap()
            .id
            .clone()
    }

    fn explain_selection(&self) -> String {
        format!(
            "PoS: Selected validator with highest stake.\nValidators: {:?}",
            self.validators
        )
    }
}

// Delegated Proof of Stake implementation
struct DposConsensus {
    validators: Vec<DposValidator>,
}

impl DposConsensus {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        DposConsensus {
            validators: vec![
                DposValidator {
                    id: "Delegate1".to_string(),
                    votes: rng.gen_range(1..10),
                },
                DposValidator {
                    id: "Delegate2".to_string(),
                    votes: rng.gen_range(1..10),
                },
                DposValidator {
                    id: "Delegate3".to_string(),
                    votes: rng.gen_range(1..10),
                },
            ],
        }
    }
}

impl Consensus for DposConsensus {
    fn select_validator(&self) -> String {
        self.validators
            .iter()
            .max_by_key(|v| v.votes)
            .unwrap()
            .id
            .clone()
    }

    fn explain_selection(&self) -> String {
        format!(
            "DPoS: Selected delegate with most votes.\nValidators: {:?}",
            self.validators
        )
    }
}

fn main() {
    println!("Consensus Mechanism Simulation\n");

    // Simulate PoW
    let pow = PowConsensus::new();
    println!("=== Proof of Work (PoW) ===");
    println!("Selected Validator: {}", pow.select_validator());
    println!("{}\n", pow.explain_selection());

    // Simulate PoS
    let pos = PosConsensus::new();
    println!("=== Proof of Stake (PoS) ===");
    println!("Selected Validator: {}", pos.select_validator());
    println!("{}\n", pos.explain_selection());

    // Simulate DPoS
    let dpos = DposConsensus::new();
    println!("=== Delegated Proof of Stake (DPoS) ===");
    println!("Selected Validator: {}", dpos.select_validator());
    println!("{}\n", dpos.explain_selection());
}
