use block::Blockchain;

mod block;

fn main() {
    let mut b = Blockchain::new();
    b.add_block("data1".to_string());
    b.add_block("data2".to_string());
    b.add_block("data3".to_string());
    dbg!(b);
}
