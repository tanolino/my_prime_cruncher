mod single_threaded;
mod multi_threaded;

use std::time::{Instant};

fn test_through() {
    let mut limit: u64 = 1;

    for _ in 0..10 {
        limit *= 10;
        println!("Test {}", &limit);
        {
            let time_start = Instant::now();
            let prims_s = single_threaded::prim_count(&limit);
            let time_passed = time_start.elapsed();
            println!("Single Threaded : counted {} primes below {}", prims_s.len(), limit);
            println!("Time: {:?}", time_passed);
        }
        {
            let time_start = Instant::now();
            let prims_m = multi_threaded::prim_count(limit);
            let time_passed = time_start.elapsed();
            println!("Multi Threaded : counted {} primes below {}", prims_m.len(), limit);
            println!("Time: {:?}", time_passed);
        }
        println!("");
    }
}

fn main() {
    println!("Starting...");
    test_through();
}
