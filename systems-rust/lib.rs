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
// Hash 3118
// Hash 9053
// Hash 3188
// Hash 7877
// Hash 9088
// Hash 1266
// Hash 2867
// Hash 3385
// Hash 4407
// Hash 2939
// Hash 4104
// Hash 2216
// Hash 2984
// Hash 8428
// Hash 2492
// Hash 1847
// Hash 8148
// Hash 8845
// Hash 5718
// Hash 9469
// Hash 7362
// Hash 2254
// Hash 5882
// Hash 8940
// Hash 2774
// Hash 3144
// Hash 4447
// Hash 1038
// Hash 1001
// Hash 1020
// Hash 1582
// Hash 1439
// Hash 3503
// Hash 5487
// Hash 1101
// Hash 8310
// Hash 7021
// Hash 7042
// Hash 7874
// Hash 4537
// Hash 7425
// Hash 3988
// Hash 2607
// Hash 7811
// Hash 9975
// Hash 4432
// Hash 6827
// Hash 2463
// Hash 6333
// Hash 5240
// Hash 3708
// Hash 5298
// Hash 8147
// Hash 1492
// Hash 9212
// Hash 1823
// Hash 5246
// Hash 2713
// Hash 5284
// Hash 2018
// Hash 3613
// Hash 7068
// Hash 3479
// Hash 8008
// Hash 5854
// Hash 7301
// Hash 1534
// Hash 9269
// Hash 4097
// Hash 9579
// Hash 6740
// Hash 8593
// Hash 2314
// Hash 7211
// Hash 6522
// Hash 3000
// Hash 3073
// Hash 5868
// Hash 1476
// Hash 1193
// Hash 4127
// Hash 7614
// Hash 9847
// Hash 4546
// Hash 9783
// Hash 1403
// Hash 4382
// Hash 1361
// Hash 8183
// Hash 2942
// Hash 8134
// Hash 8736
// Hash 8426
// Hash 6303
// Hash 3419
// Hash 7625
// Hash 6226
// Hash 9694
// Hash 1964
// Hash 4375
// Hash 5302
// Hash 2785
// Hash 4542