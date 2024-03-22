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

    let now = Instant::now();

    let mut i = 0;
    while i < 50 {
        ljm_wrapper.read_name(open_call, "AIN0".to_string()).expect("");
        ljm_wrapper.read_name(open_call, "AIN1".to_string()).expect("");

        // thread::sleep(Duration::from_millis(1));
        i += 1;
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn main() {
    stream();
}
