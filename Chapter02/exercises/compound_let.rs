fn main() {
    let mut a = 5;
    let mut b = 6;
    let n = 7;
    
    let a = b = n;
    println!("{:?}{:?}{:?}", a, b, n); // ()77

    // no swap :
    let mut c = 5;
    let mut d = 6;
    let c = d = c;
    println!("{:?}{:?}", c, d);  // ()5
}
// a gets the value of the expression:  b = n;
// the value of that expression is ()
// ()77
// ()5