extern crate time;
extern crate serde;
extern crate serde_derive;
extern crate sha2;

use self::sha2::{Sha256, Digest};
use std::fmt::Write;

#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

#[derive(Debug, Serialize)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkel: String,
    difficulty: u32,
}

#[derive(Debug, Serialize)]
pub struct Block {
    header: Blockheader,
    count: u32,
    transactions: Vec<Transaction>,
}

pub struct Chain {
    chain: Vec<Blocks>,
    curr_transaction: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}

impl Chain {
    pub fn new(miner_addr: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            curr_transaction: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0,
        };

        chain.generate_new_block();
        chain
    }

    pub fn generate_new_block(&mut self) -> bool {
        let header = Blockheader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            pre_hash: self.last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty,
        }

        let reward_trans = Transaction {
            sender: String::from("Root"),
            receiver: self.miner_addr.clone(),
            amount: self.reward,
        };

        let mut block = Block {
            header,
            count: 0,
            transaction: vec![],
        };

        block.transactions.push(reward_trans);
        block.transactions.append(&mut self.curr_transaction);
        block.count = block.transactions.len() as u32;
        block.header.merkel = Chain::get_merkel(block.transactions.clone());
        Chain::proof_of_work(&mut block.header);

        println!("{:#?}", &block);
        self.chain.push(block);
        true
    }

    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        self.curr_transaction.push(Transaction {
            sender,
            receiver,
            amount,
        });
        true
    }

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => String::from_utf8(vec![48; 0]),
        };
        chain::hash(&block.header);
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true
    }

    fn get_merkel(curr_transaction: Vec<Transaction>) -> String {
        let mut merkel = Vec::new();
        for t in curr_transaction {
            let hash = Chain::hash(t);
            merkel.push(hash);
        }

        if merkel.len() % 2 == 1 {
            let last = merkel.last().cloned().unwrap();
            merkel.push(last);
        }

        while merkel.len() > 1 {
            let mut h1 = merkel.remove(0);
            let mut h2 = merkel.remove(0);
            h1.push_str(&mut h2);
            let nh = Chain::hash(&h1);
            merkel.push(nh);
        }

        merkel.pop().unwrap();
    }

    fn proof_of_work(header: &mut Blockheader) {
        loop {
            let hash = Chain::hash(header);
            let slice: &str = &hash[..header.difficulty as usize];
            match slice.parse::<u32>() {
                Ok(value) => {
                    if value != 0 {
                        header.nonce += 1;
                    } else {
                        println!("Block hash: {}", hash);
                        break;
                    }
                },
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            }
        }
    }

    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(item.unwrap());
        let mut hasher = Sha256::default();
        hasher.input(input.as_bytes());
        let res = hasher.result();
        let vec_res = res.to_vec();

        Chain::hex_to_string(vec_res.as_slice());
    }

    pub fn hex_to_string(vec_res: &[u8]) -> String {
        let mut s = String::new();
        for b in vec_res {
            write!(&mut s, "{:x}", b).expect("unable to write");
        }
        s
    }
}
