extern crate ljmrs;

use ljmrs::LJMWrapper;

fn read() {
    let ljm_wrapper = unsafe { LJMWrapper::init(None) }.unwrap();

    let open_call = ljm_wrapper
        .open_jack(
            ljmrs::DeviceType::ANY,
            ljmrs::ConnectionType::ANY,
            "-2".to_string(),
        )
        .expect("Could not open DEMO LabJack");

    println!("Opened LabJack, got handle: {}", open_call);

    let read_value = ljm_wrapper
        .get_config("LJM_STREAM_SCANS_RETURN".to_string())
        .expect("Expected Value");

    println!("Got configuration value: {}", read_value);

    ljm_wrapper
        .set_config("LJM_STREAM_SCANS_RETURN".to_string(), 2)
        .expect("Expected Value");

    println!("Set config value to 2, reading...");

    let read_value = ljm_wrapper
        .get_config("LJM_STREAM_SCANS_RETURN".to_string())
        .expect("Expected Value");

    println!("Got new configuration value: {}", read_value);
}

fn main() {
    read();
}
