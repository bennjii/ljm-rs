extern crate ljmrs;

use std::time::Instant;
use ljmrs::LJMWrapper;

fn load() {
    let now = Instant::now();

    let ljm_wrapper = unsafe { LJMWrapper::init() }.unwrap();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();

    let (addr, typ) = ljm_wrapper.name_to_address("AIN0".to_string()).expect("Expected NTA");

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();

    ljm_wrapper.write_name(-2, "AIN0_RANGE".to_string(), 15_u32).expect("Expected NTA");

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("Function result: {}:{}", addr, typ);
}

fn main() {
    load();
}