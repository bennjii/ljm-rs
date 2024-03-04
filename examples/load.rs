extern crate ljmrs;

use ljmrs::LJMWrapper;
use std::time::Instant;

fn load() {
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

    let (addr, typ) = ljm_wrapper
        .name_to_address("TEST_INT32")
        .expect("Expected NTA");
    println!("TEST_INT32 => Address: {}, Type: {}", addr, typ);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn main() {
    load();
}
