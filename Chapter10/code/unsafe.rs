use std::mem;

static mut N: i32 =  42;

fn main() {
	let v: &[u8] = unsafe { 
		mem::transmute("Gandalf") 
	};
	println!("{:?}", v);

    // N = 108; // use of mutable static requires unsafe function or block
    // reading or changing a static mutable variable:
    unsafe {
       println!("{:?}", N );  // 42
       N = 108;
       println!("{:?}", N );  // 108
    }
}
// [71, 97, 110, 100, 97, 108, 102]
// 42
// 108