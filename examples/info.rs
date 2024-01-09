extern crate ljmrs;

use ljmrs::LJMWrapper;

fn info() {
    let ljm_wrapper = unsafe { LJMWrapper::init(None) }.unwrap();

    let open_call = ljm_wrapper
        .open_jack(
            ljmrs::DeviceType::ANY,
            ljmrs::ConnectionType::ANY,
            "-2".to_string(),
        )
        .expect("Could not open DEMO LabJack");

    println!("Opened LabJack, got handle: {}", &open_call);

    let info =
        LJMWrapper::get_handle_info(&ljm_wrapper, open_call).expect("Handle verification failed.");

    println!("--- LabJack Info ---\n{}\n--- LabJack Info ---", info);

    // The C String recovery is an unsafe process
    let ip = unsafe {
        ljm_wrapper
            .number_to_ip(info.ip_address)
            .expect("Could not convert IP.")
    };
    println!("IP: {ip}");
}

fn main() {
    info();
}
