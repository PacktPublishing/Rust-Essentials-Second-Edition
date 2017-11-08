use std::thread;
use std::time;

fn main() {
    let mut health = 12;
    for i in 2..5 {
        thread::spawn(move || {
            health *= i;
        });
    }
    thread::sleep(time::Duration::from_secs(2));
    println!("{}", health); // 12
}