extern crate ljmrs;

use std::time::Instant;

use ljmrs::LJMLibrary;

fn stream() {
    unsafe { LJMLibrary::init() }.unwrap();

    let open_call = LJMLibrary::open_jack(
        ljmrs::DeviceType::ANY,
        ljmrs::ConnectionType::ANY,
        "-2".to_string(), // Use "ANY" for physical hardware
    ).expect("Could not open DEMO LabJack");

    println!("Opened LabJack, got handle: {}", open_call);

    let now = Instant::now();

    for _ in 0..50 {
        LJMLibrary::read_name(open_call, "AIN0").expect("");
        LJMLibrary::read_name(open_call, "AIN1").expect("");
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn main() {
    stream();
}
