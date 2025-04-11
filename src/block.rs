use super::*;
use sha2::{Digest, Sha256};
use std::{io::Read, time::SystemTime};

#[derive(Debug)]
pub struct Block {
    timestamp: u128,
    data: String,
    prev_block_hash: String,
    hash: String,
}

impl Block {
    /// NewBlock creates and returns Block
    pub fn new_block(data: String, prev_block_hash: String) -> Result<Block> {
        let mut block = Block {
            timestamp: 0,
            data,
            prev_block_hash,
            hash: String::new(),
        };
        block.set_hash()?;
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
        let bytes = bincode::serde::encode_to_vec(&content, bincode::config::legacy())?;
        let mut hasher = Sha256::new();
        hasher.update(&bytes[..]);
        self.hash = format!("{:x}", hasher.finalize());
        Ok(())
    }

    pub fn get_hash(&self) -> String{
        self.hash.clone()
    }
}
