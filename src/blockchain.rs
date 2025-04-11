use super::*;
use crate::block::Block;
/// Blockchain keeps a sequence of Blocks
#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    /// AddBlock saves provided data as a block in the blockchain
    pub fn add_block(&mut self, data: String) -> Result<()> {
        let prev = self.blocks.last().unwrap();
        let prev_block_hash = prev.get_hash();
        let newblock = Block::new_block(data, prev_block_hash)?;
        self.blocks.push(newblock);
        Ok(())
    }
    /// NewGenesisBlock creates and returns genesis Block
    pub fn new_genesis_block() -> Block {
        Block::new_block(String::from("Genesis Block"), String::new()).unwrap()
    }

    pub fn new() -> Blockchain {
        Blockchain {
            blocks: vec![Blockchain::new_genesis_block()],
        }
    }
}
