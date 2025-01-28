use std::sync::{Arc, Mutex};
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction { pub sender: String, pub receiver: String, pub amount: f64 }

pub trait Validator {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str>;
    fn process_block(&mut self, block: ConsensusBlock) -> bool;
}

pub struct NodeState {
    pub chain: Vec<ConsensusBlock>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl Validator for NodeState {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str> {
        // Cryptographic verification logic
        Ok(true)
    }
    fn process_block(&mut self, block: ConsensusBlock) -> bool {
        self.chain.push(block);
        true
    }
}

// Hash 5183
// Hash 5976
// Hash 6182
// Hash 7603
// Hash 7090
// Hash 4538
// Hash 7438
// Hash 4121
// Hash 3192
// Hash 5505
// Hash 3521
// Hash 3126
// Hash 1689
// Hash 4572
// Hash 4179
// Hash 7133
// Hash 6680
// Hash 2695
// Hash 1918
// Hash 2102
// Hash 9995
// Hash 8263
// Hash 5322
// Hash 5208
// Hash 4200
// Hash 6735
// Hash 9315
// Hash 8557
// Hash 1039
// Hash 9594
// Hash 3747
// Hash 9944
// Hash 9060
// Hash 6837
// Hash 3658
// Hash 7670
// Hash 2368
// Hash 3466
// Hash 9092
// Hash 5664
// Hash 2723
// Hash 5094
// Hash 2655
// Hash 8750
// Hash 1787
// Hash 4234
// Hash 6759
// Hash 1493
// Hash 2228
// Hash 5217
// Hash 3304
// Hash 5453
// Hash 3884
// Hash 7764
// Hash 2810
// Hash 5236
// Hash 6903
// Hash 7777
// Hash 2575
// Hash 1652
// Hash 7910
// Hash 8732
// Hash 6995
// Hash 3968
// Hash 9878
// Hash 4839
// Hash 8788
// Hash 9255
// Hash 3304
// Hash 1584
// Hash 4479
// Hash 9932
// Hash 6944