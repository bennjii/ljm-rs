extern crate ljmrs;

use std::thread::sleep;
use std::time::{Duration, Instant};

use ljmrs::LJMLibrary;

fn camera() {
    let now = Instant::now();

    let open_call = LJMLibrary::open_jack(
        ljmrs::DeviceType::ANY,
        ljmrs::ConnectionType::ANY,
        "ANY".to_string(),
    )
    .expect("Could not open DEMO LabJack");

    println!("Opened LabJack, got handle: {}", open_call);

    // Will set a 20s timer on taking a photo using a camera
    // which is attached to the following relay.
    let relay = "FIO0";

    println!("20s!");
    LJMLibrary::write_name(open_call, relay, 0).expect("Expected Value");
    sleep(Duration::from_secs(10));

    println!("10s!");
    LJMLibrary::write_name(open_call, relay, 0).expect("Expected Value");
    sleep(Duration::from_secs(10));

    println!("0s!");
    LJMLibrary::write_name(open_call, relay, 1).expect("Expected Value");
    sleep(Duration::from_secs(10));

    println!("Closing...");
    LJMLibrary::write_name(open_call, relay, 0).expect("Expected Value");
    sleep(Duration::from_secs(2));

    println!("Done.");
}

fn main() {
    camera();
}
