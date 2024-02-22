pub struct Block {
    timestamp: u128,
    transaction: String,
    prev_hash: String,
    hash: String,
    height: usize,
    nonce: i32,
}

pub struct Blockchain {
    blocks: Vec<Block>, // Vector of Block
}

impl Block {
    pub fn new_block(data: String, prev: String, height: usize) -> Result<Block> {
        let timestamp: i128 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).as_millis();

        let mut block = Block {
            timestamp: timestamp,
            transaction: data,
            prev_hash: prev,
            hash: String::new(),
            height,
            nonce: 0,
        };

        block.run_proof_if_work()?;
        OK(block)
    }

    fn run_proof_if_work(&mut self) -> Result<()> {
        info!("Mining the  block");
        while !self.validate()? {
            self.nonce += 1;
        }

        let data: Vec<u8> = self.prepare_hash_data()?;
        let mut hasher: Sha256 = Sha256::new();
        hasher.input(&data[..]);
        self.hash = hasher.result_str();
        OK(())
    }
}
