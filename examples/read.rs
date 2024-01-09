extern crate ljmrs;

use ljmrs::LJMWrapper;
use std::time::Instant;

fn read() {
    let now = Instant::now();

    let ljm_wrapper = unsafe { LJMWrapper::init(None) }.unwrap();

    let open_call = ljm_wrapper
        .open_jack(
            ljmrs::DeviceType::ANY,
            ljmrs::ConnectionType::ANY,
            "-2".to_string(),
        )
        .expect("Could not open DEMO LabJack");

    println!("Opened LabJack, got handle: {}", open_call);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();

    let read_value = ljm_wrapper
        .read_name(open_call, "TEST_INT32".to_string())
        .expect("Expected Value");
    println!("Got: {}", read_value);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    ljm_wrapper
        .write_name(open_call, "TEST_INT32".to_string(), 15)
        .expect("Expected Value");

    let now = Instant::now();

    let read_value = ljm_wrapper
        .read_name(open_call, "TEST_INT32".to_string())
        .expect("Expected Value");
    println!("Got: {}", read_value);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn main() {
    read();
}
