use std::thread;

fn main() {
	let result = thread::spawn(move || {
    	panic!("I have fallen into an unrecoverable trap!");
	}).join();

	if result.is_err() {
		println!("This child has panicked");
	}
}
// thread '<unnamed>' panicked at 'I have fallen into an unrecoverable trap!'
// This child has panicked
