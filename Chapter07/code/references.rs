fn main() {
	// immutable reference to immutable values:
	let n = 42i32;

	// a reference to n:
	let m = &n;
	println!("The address of n is {:p}", m);
	println!("The value of n is {}", *m);
	println!("The value of n is {}", m);

	let q = &42;
	println!("The square is: {}", square(q)); // 1764

	fn square(k: &i32) -> i32 {
    	*k * *k
	}

	// references to immutable values are immutable:
	// *m = 7; // error: cannot assign to immutable borrowed content `*m`

	// multiple references to an immutable value:
	let o = &n;
	println!("The address of n is {:p}", o);
	println!("The value of n is {}", *o);

	// a mutable reference to an immutable value is not possible:
	// let m = &mut n; // error: cannot borrow immutable local variable `n` as mutable

	// references to mutable values:
	let mut u = 3.14f64;
	let v = &mut u;
	println!("The address of u is {:p}", v);
	println!("The value of u is {}", *v);
	*v = 3.15;
	println!("The value of u is now {}", *v);
	// error: cannot borrow `u` as immutable because it is also borrowed as mutable
	// println!("The value of u is {}", u);  
	// u = u * 2.0; // error: cannot assign to `u` because it is borrowed
	
	// more than 1 mutable reference is not allowed:
	// let w = &mut u; // error: cannot borrow `u` as mutable more than once at a time

	// change a value by passing it as a reference to a function:
	let mut m = 7;
    add_three_to_magic(&mut m);
    println!("m is now {}", m);  // 10
}

fn add_three_to_magic(num: &mut i32) {
    *num += 3;  // value is changed in place with +=, is same as:  *num = *num + 3
}
// The address of n is 0x7078eff8bc
// The value of n is 42
// The value of n is 42
// The square is: 1764
// The address of n is 0x7078eff8bc
// The value of n is 42
// The address of u is 0x7078effa38
// The value of u is 3.14
// The value of u is now 3.15
// m is now 10