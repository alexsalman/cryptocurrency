
use hello_rust::Block;
use hello_rust::Hashable;
// fn blockObj_data(self)
// {
//     return self.data;
// }
// fn blockObj_prev(self)
// {
//     return self.prev;
// }
// fn hash(self)
// {
//     let add = self.id + self.prev;
//     for i in self.data.len()
//     {
//         a = a + self.data[i];
//     }
//     return add
// }
// fn printBlock(self)
// {
//     println!(self.id);
//     println!(self.data);
// }

// struct Blockchain
// {
//     blocks: list
// }
// fn appendBlock(struct String) -> String
// {
//     if self.blocks.len() == 0
//     {
//         self.blocks.push(BlockObj(text, self.blocks.len(), 0));
//     }
//     else
//     {
//         self.blocks.push(BlockObj(text, self.blocks.len(), hash(self.blocks.len()-1]));
//     }
// }
// fn getBlock(struct)-> i32
// {
//     return self.blocks[i];
// }
// fn size()-> i32
// {
//     return self.blocks.len()
// }
// fn removeLast(struct String)
// {
//     self.blocks.last().unwrap();
// }
// fn printBlockchain()-> i32
// {
//     for i in self.blocks
//         i.printBlock();
// }
// fn main()
// {
//     let myChain = Blockchain();
//     myChain.appendBlock("first")
//     myChain.appendBlock("second")
//     myChain.appendBlock("third")
//     myChain.printBlockchain()
// };

fn main(){
    let mut block = Block::new(0, 0, vec![0; 32], 0, "What a block".to_owned());//hehehe
    println!("{:?}", &block);
    let h = block.hash();
    println!("{:?}", &h);
    block.hash = h;
    println!("{:?}", &block);
}