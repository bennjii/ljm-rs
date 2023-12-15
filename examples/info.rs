extern crate ljmrs;

use ljmrs::LJMWrapper;

fn info() {
    let ljm_wrapper = unsafe { LJMWrapper::init() }.unwrap();

    let open_call = ljm_wrapper.open_jack(
        ljmrs::DeviceType::ANY,
        ljmrs::ConnectionType::ANY,
        "-2".to_string(),
    ).expect("Could not open DEMO LabJack");

    println!("Opened LabJack, got handle: {}", open_call);

    let info = LJMWrapper::get_handle_info(&ljm_wrapper, open_call.clone()).expect("Handle verification failed.");
    let ip_address = LJMWrapper::number_to_ip(&ljm_wrapper, info.ip_address).expect("Unable to retrieved IP");

    println!("Got IP, {}", ip_address);
}

fn main() {
    info();
}