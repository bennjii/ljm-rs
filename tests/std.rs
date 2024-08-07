use std::time::Instant;

use ljmrs;
use ljmrs::LJMLibrary;

#[test]
fn open() {
    let _ = unsafe { LJMLibrary::init(None) };

    let open_call = LJMLibrary::open_jack(
        ljmrs::DeviceType::ANY,
        ljmrs::ConnectionType::ANY,
        "-2".to_string(),
    );

    assert!(open_call.is_ok());
}

#[test]
fn get_name() {
    let _ = unsafe { LJMLibrary::init(None) };

    let mut elapsed_times: Vec<f32> = vec![];
    let addresses = vec!["AIN0", "AIN1", "AIN2", "AIN3", "FIO0", "FIO1"];

    for _ in 0..addresses.len() {
        let now = Instant::now();
        let result = LJMLibrary::name_to_address("AIN0".to_string());
        assert!(result.is_ok());

        let elapsed = now.elapsed().as_millis();
        elapsed_times.push(elapsed as f32)
    }

    let avg: f32 = elapsed_times.iter().sum::<f32>() / addresses.len() as f32;
    assert!(avg < 5.0f32, "Average time elapsed: {}. Computes: {:?}", avg, elapsed_times);
}

#[test]
fn standard_open_read() {
    let _ = unsafe { LJMLibrary::init(None) };

    let open_call = LJMLibrary::open_jack(
        ljmrs::DeviceType::ANY,
        ljmrs::ConnectionType::ANY,
        "-2".to_string(),
    ).expect("Could not open DEMO LabJack");

    assert_ne!(open_call, -1);

    let read_value = LJMLibrary::read_name(open_call, "TEST_INT32".to_string())
        .expect("Expected Value");

    assert_eq!(read_value, 0f64);
}
