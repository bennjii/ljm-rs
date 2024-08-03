extern crate ljmrs;

use ljmrs::LJMWrapper;

fn library() {
    let unique_library_location = "/usr/lib/ljm/libLabJackM.dylib".to_string();
    unsafe { LJMWrapper::init(Some(unique_library_location)) }.unwrap();

    let open_call = LJMWrapper::open_jack(
        ljmrs::DeviceType::ANY,
        ljmrs::ConnectionType::ANY,
        "-2".to_string(),
    ).expect("Could not open DEMO LabJack");

    println!("Opened LabJack, got handle: {}", &open_call);
}

fn main() {
    library();
}
