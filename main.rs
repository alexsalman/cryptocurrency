
use hello_rust::Block;
use hello_rust::Hashable;

fn main(){
    let mut block = Block::new(0, 0, vec![0; 32], 0, "What a block".to_owned());//hehehe
    println!("{:?}", &block);
    let h = block.hash();
    println!("{:?}", &h);
    block.hash = h;
    println!("{:?}", &block);
}