use std::time::Instant;
use ljmrs::LJMWrapper;

fn read() {
    let now = Instant::now();

    let ljm_wrapper = unsafe { LJMWrapper::init() }.unwrap();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();

    let (addr, typ) = ljm_wrapper.name_to_address("AIN0".to_string()).expect("Expected AIN0");

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();

    ljm_wrapper.read_name(-2, "AIN0".to_string()).expect("Expected read to succeed");

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("Function result: {}:{}", addr, typ);
}

fn main() {
    read();
}
