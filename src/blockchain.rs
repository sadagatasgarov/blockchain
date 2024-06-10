use crate::block::Block;
use crate::errors::Result;

const TARGET_HEXT: usize = 4;

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blocks: vec![Block::new_genesis_block()],
        }
    }

    pub fn add_block(&mut self, data: String) -> Result<()> {
        let prev = self.blocks.last().unwrap();
        let new_block = Block::new_block(data, prev.get_hash(), TARGET_HEXT)?;
        self.blocks.push(new_block);
        Ok(())
    }
}
