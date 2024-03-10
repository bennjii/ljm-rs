extern crate ljmrs;

use ljmrs::LJMWrapper;

fn stream() {
    let mut ljm_wrapper = unsafe { LJMWrapper::init(None) }.unwrap();

    let open_call = ljm_wrapper
        .open_jack(
            ljmrs::DeviceType::ANY,
            ljmrs::ConnectionType::ANY,
            "-2".to_string(),
        )
        .expect("Could not open DEMO LabJack");

    println!("Opened LabJack, got handle: {}", open_call);

    let streams = vec!["AIN0", "AIN1"];

    // Do note that this examples will fail on
    // DEMO mode labjacks, as currently stream mode
    // is not supported for them.
    ljm_wrapper
        .stream_start(open_call, 1, 1000.0, streams)
        .expect("Failed to start stream");

    assert!(ljm_wrapper.is_stream_active());

    let read_value = ljm_wrapper
        .stream_read(open_call)
        .expect("Could not read values");

    println!("Got: {:?}", read_value);

    ljm_wrapper.stream_stop(open_call).expect("Expected Value");

    assert!(!ljm_wrapper.is_stream_active());
}

fn main() {
    stream();
}
