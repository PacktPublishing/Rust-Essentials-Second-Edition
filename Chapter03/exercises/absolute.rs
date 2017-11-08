fn main() {
    assert_eq!(5, abs(-5));
}

// error version:
// fn abs(x: i32) -> u32 {
//    if x > 0 { x }
//    else { -x }
// }
// lines 6 and 7:
// error[E0308]: mismatched types: expected u32, found i32

// corrected version:
fn abs(x: i32) -> i32 {
    if x > 0 { 
   	    x 
    } else { 
   	    -x 
    }
}