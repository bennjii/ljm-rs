extern crate ljmrs;

use ljmrs::LJMLibrary;

fn library() {
    #[cfg(feature = "dynlink")]
    {
        // We can specify the exact runtime location to fetch our library from.
        let unique_library_location = "/usr/lib/ljm/libLabJackM.dylib".to_string();
        unsafe { LJMLibrary::with_path(unique_library_location) }.unwrap();
    }
    #[cfg(feature = "staticlink")]
    unsafe { LJMLibrary::init() }.unwrap();

    let open_call = LJMLibrary::open_jack(
        ljmrs::DeviceType::ANY,
        ljmrs::ConnectionType::ANY,
        "-2".to_string(),
    )
    .expect("Could not open DEMO LabJack");

    println!("Opened LabJack, got handle: {}", &open_call);
}

fn main() {
    library();
}
