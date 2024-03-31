extern crate ljmrs;

use std::time::Instant;

use ljmrs::LJMWrapper;

fn stream() {
    let mut ljm_wrapper = unsafe { LJMWrapper::init(None) }.unwrap();

    let open_call = ljm_wrapper
        .open_jack(
            ljmrs::DeviceType::ANY,
            ljmrs::ConnectionType::ANY,
            "ANY".to_string(),
        )
        .expect("Could not open DEMO LabJack");

    println!("Opened LabJack, got handle: {}", open_call);

    let streams = vec!["AIN0"];

    // Do note that this examples will fail on
    // DEMO mode labjacks, as currently stream mode
    // is not supported for them.
    ljm_wrapper
        .stream_start(open_call, 2, 50_000.0, streams)
        .expect("Failed to start stream");

    assert!(ljm_wrapper.is_stream_active());

    let now = Instant::now();

    let mut i = 0;
    while i < 50 {
        let read_value = ljm_wrapper
            .stream_read(open_call)
            .expect("Could not read values");

        println!("Got {}: {:?}", read_value.len(), read_value);

        // thread::sleep(Duration::from_millis(1));
        i += 1;
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    ljm_wrapper.stream_stop(open_call).expect("Expected Value");

    assert!(!ljm_wrapper.is_stream_active());
}

fn main() {
    stream();
}
