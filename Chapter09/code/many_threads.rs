use std::thread;
use std::time;

static NTHREADS: i32 = 10000;

fn main() {
    println!("************************** Before the start of the threads");
    for i in 0..NTHREADS {
        thread::spawn(move || {
            println!("this is thread number {}", i)
        });
    }
    thread::sleep(time::Duration::from_millis(500));
    println!("************************** All threads finished!");
}