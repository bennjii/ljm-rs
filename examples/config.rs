extern crate ljmrs;

use ljmrs::LJMLibrary;

fn read() {
    unsafe { LJMLibrary::init(None) }.unwrap();

    let open_call = LJMLibrary::open_jack(
        ljmrs::DeviceType::ANY,
        ljmrs::ConnectionType::ANY,
        "-2".to_string(),
    ).expect("Could not open DEMO LabJack");

    println!("Opened LabJack, got handle: {}", open_call);

    let read_value = LJMLibrary::get_config("LJM_STREAM_SCANS_RETURN".to_string())
        .expect("Expected Value");

    println!("Got configuration value: {}", read_value);

    LJMLibrary::set_config("LJM_STREAM_SCANS_RETURN".to_string(), 2)
        .expect("Expected Value");

    println!("Set config value to 2, reading...");

    let read_value = LJMLibrary::get_config("LJM_STREAM_SCANS_RETURN".to_string())
        .expect("Expected Value");

    println!("Got new configuration value: {}", read_value);
}

fn main() {
    read();
}
