use crate::{ block, Block, Hashable };

pub struct BlockChain {
    pub blocks: Vec<Block>,
}
impl BlockChain {
    //
    pub fn verify(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            if block.index != (i as u32) {
                print!("Index mismstach {} !- {}", block.index, i);
                return false;
            } else if !block::check_difficulty(&block.hash(), block.difficulty) {
                println!("Difficulty Failure");
                return false;
            } else if i != 0 {
                let prev_block = &self.blocks[i - 1];
                if block.timestamp <= prev_block.timestamp {
                    println!(" Time did not increase to {}", prev_block.timestamp);
                    return false;
                } else if block.prev_block_hash != prev_block.hash {
                    println!("Hash did not Match");
                    return false;
                }
            } else {
                if block.prev_block_hash != vec![0; 32] {
                    println!("Gebesus block prev_block_hash incorrect");
                    return false;
                }
            }
        }
        true
    }
}
