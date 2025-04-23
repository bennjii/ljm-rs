extern crate ljmrs;

use std::time::Instant;

use ljmrs::LJMLibrary;

fn stream() {
    #[cfg(feature = "dynlink")]
    unsafe { LJMLibrary::init(None) }.unwrap();
    #[cfg(feature = "staticlink")]
    unsafe { LJMLibrary::init() }.unwrap();

    let open_call = LJMLibrary::open_jack(
        ljmrs::DeviceType::ANY,
        ljmrs::ConnectionType::ANY,
        "-2".to_string(), // Use "ANY" for physical hardware
    )
    .expect("Could not open DEMO LabJack");

    println!("Opened LabJack, got handle: {}", open_call);

    let streams = vec!["AIN0"];

    // Do note that this examples will fail on
    // DEMO mode labjacks, as currently stream mode
    // is not supported for them.
    LJMLibrary::stream_start(open_call, 2, 50_000.0, streams).expect("Failed to start stream");

    assert!(LJMLibrary::is_stream_active(open_call));

    let now = Instant::now();

    let mut i = 0;
    while i < 50 {
        let read_value = LJMLibrary::stream_read(open_call).expect("Could not read values");

        println!("Got {}: {:?}", read_value.len(), read_value);
        i += 1;
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    LJMLibrary::stream_stop(open_call).expect("Expected Value");

    assert!(!LJMLibrary::is_stream_active(open_call));
}

fn main() {
    stream();
}
