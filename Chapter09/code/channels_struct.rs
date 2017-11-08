use std::thread;
use std::sync::mpsc::channel;

struct Block {
    value: i32
}

fn main() {
    let (tx1, rx1) = channel::<Block>();
    let (tx2, rx2) = channel::<Block>();
    
    thread::spawn(move|| {
        let mut block = rx1.recv().unwrap();
        println!("Input: {:?}", block.value);
        
        block.value += 1;
        tx2.send(block).unwrap();
    });
    
    let input = Block{ value: 1 };
    tx1.send(input).unwrap();
    
    let output = rx2.recv().unwrap();
    println!("Output: {:?}", output.value);
}
// Input: 1
// Output: 2