extern crate ljmrs;

use std::time::Instant;

use ljmrs::LJMWrapper;

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

    let name: &str = "FIO0";

    let (addr, typ) = ljm_wrapper
        .name_to_address(name)
        .expect("Expected NTA");
    println!("{name} => Address: {}, Type: {}", addr, typ);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn main() {
    load();
}
