use std::f32;

struct Person {
	name: &'static str,
	id:   i32
}

struct Pair<T> {
    first: T,
    second: T,
}

fn main() {
	// generic structs:
	let magic_pair: Pair<u32> = Pair { first: 7, second: 42 };
	let pair_of_magicians: Pair<&str> = Pair { first: "Gandalf", second: "Sauron" };
	let a = second(magic_pair);

	// using Option<T>
	let x: Option<i8> = Some(5);
	let pi: Option<f64> = Some(3.14159265359);
	let none: Option<f64> = None;
	let none2 = None::<f64>;
	let name: Option<&str> = Some("Joyce");
	// let magic: Option<f32> = Some(42); // error: mismatched types

	let p1 = Person{ name: "James Bond", id: 7 };
	let p2 = Person{ name: "Vin Diesel", id: 12 };
	let p3 = Person{ name: "Robin Hood", id: 42 };
	let op1: Option<Person> = Some(p1);
	let pvec: Vec<Person> = vec![p2, p3]; // type annotation is not necessary
	// let pvec = vec![p2, p3];
}

fn second<T>(pair: Pair<T>) {
	pair.second;
}
