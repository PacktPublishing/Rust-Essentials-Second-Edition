extern crate num_cpus;
extern crate threadpool;

use std::thread;
use std::time;
use threadpool::ThreadPool;

fn main() {
    let ncpus = num_cpus::get();
    println!("The number of cpus in this machine is: {}", ncpus);

    let pool = ThreadPool::new(ncpus);

    for i in 0..ncpus {
    	pool.execute(move || {
        	println!("this is thread number {}", i)
        });
    }

    thread::sleep(time::Duration::from_millis(50));
}
// The number of cpus in this machine is: 8
// for example (output can change each run)
// this is thread number 0
// this is thread number 5
// this is thread number 7
// this is thread number 3
// this is thread number 4
// this is thread number 1
// this is thread number 6
// this is thread number 2