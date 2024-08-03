extern crate ljmrs;

use std::time::Instant;

use ljmrs::LJMWrapper;

fn read() {
    let now = Instant::now();

    unsafe { LJMWrapper::init(None).expect("Should have found library") };

    let open_call = LJMWrapper::open_jack(
        ljmrs::DeviceType::ANY,
        ljmrs::ConnectionType::ANY,
        "-2".to_string(),
    )
        .expect("Could not open DEMO LabJack");

    println!("Opened LabJack, got handle: {}", open_call);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();

    let read_value = LJMWrapper::read_name(open_call, "TEST_INT32")
        .expect("Expected Value");
    println!("Got: {}", read_value);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    LJMWrapper::write_name(open_call, "TEST_INT32", 15)
        .expect("Expected Value");

    let now = Instant::now();

    let read_value = LJMWrapper::read_name(open_call, "TEST_INT32")
        .expect("Expected Value");
    println!("Got: {}", read_value);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn main() {
    read();
}
