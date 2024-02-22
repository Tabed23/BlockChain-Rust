use blockchain_rust::{ now, Block };
use blockchain_rust::Hashable;
fn main() {
    let mut block: Block = Block::new(0, now(), vec![0; 32], 0, "GenesisBlock1".to_owned());
    println!("{:?}", &block);

    let h = block.hash();
    println!("{:?}", &h);
    block.hash = h;

    println!("{:?}", &block);
}
