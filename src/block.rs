use super::*;
use sha2::{Digest, Sha256};
use std::{io::Write, time::SystemTime};
const TARGET_HEXS: usize = 4;
const SERIALIZE_CONFIG: bincode::config::Configuration<
    bincode::config::LittleEndian,
    bincode::config::Fixint,
    bincode::config::NoLimit,
> = bincode::config::legacy();
#[derive(Debug)]
pub struct Block {
    timestamp: u128,
    data: String,
    prev_block_hash: String,
    hash: String,
    /// add nonce
    nonce: i64,
}

impl Block {
    /// NewBlock creates and returns Block
    pub fn new_block(data: String, prev_block_hash: String, nonce: i64) -> Result<Block> {
        let mut block = Block {
            timestamp: 0,
            data,
            prev_block_hash,
            hash: String::new(),
            nonce,
        };
        block.set_hash()?;
        block.run_proof_of_work()?;
        Ok(block)
    }
    /// SetHash calculates and sets block hash
    pub fn set_hash(&mut self) -> Result<()> {
        self.timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis();
        let content = (
            self.data.clone(),
            self.prev_block_hash.clone(),
            self.timestamp,
        );
        let bytes = bincode::serde::encode_to_vec(&content, SERIALIZE_CONFIG)?;
        let mut hasher = Sha256::new();
        hasher.update(&bytes[..]);
        self.hash = format!("{:x}", hasher.finalize());
        Ok(())
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn get_nonce(&self) -> i64 {
        self.nonce
    }

    fn prepare_hash_data(&self) -> Result<Vec<u8>> {
        let content = (
            self.prev_block_hash.clone(),
            self.data.clone(),
            self.timestamp,
            TARGET_HEXS,
            self.nonce,
        );

        let bytes = bincode::encode_to_vec(&content, SERIALIZE_CONFIG)?;
        Ok(bytes)
    }

    /// Validate validates block's PoW
    fn validate(&self) -> Result<bool> {
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.update(data);
        let mut vec1: Vec<u8> = Vec::new();
        vec1.resize(TARGET_HEXS, '0' as u8);
        let result_hash = format!("{:x}", hasher.finalize());
        Ok(result_hash[0..TARGET_HEXS] == String::from_utf8(vec1)?)
    }

    /// Run performs a proof-of-work
    fn run_proof_of_work(&mut self) -> Result<()> {
        println!("Mining the block containing \"{}\"\n", self.data);
        while !self.validate()? {
            self.nonce += 1;
        }
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.update(&data[..]);
        self.hash = format!("{:x}", hasher.finalize());
        Ok(())
    }
}
