extern crate ljmrs;

use std::time::Instant;

use ljmrs::LJMWrapper;

fn stream() {
    unsafe { LJMWrapper::init(None) }.unwrap();

    let open_call = LJMWrapper::open_jack(
        ljmrs::DeviceType::ANY,
        ljmrs::ConnectionType::ANY,
        "-2".to_string(), // Use "ANY" for physical hardware
    ).expect("Could not open DEMO LabJack");

    println!("Opened LabJack, got handle: {}", open_call);

    let now = Instant::now();

    for _ in 0..50 {
        LJMWrapper::read_name(open_call, "AIN0").expect("");
        LJMWrapper::read_name(open_call, "AIN1").expect("");
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn main() {
    stream();
}
