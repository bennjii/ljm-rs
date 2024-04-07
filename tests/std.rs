use ljmrs;
use ljmrs::{LJMError, LJMErrorCode, LJMWrapper};

fn assert_error(error: LJMError, error_code: i32) {
    assert!(
        matches!(
            error,
            LJMError::ErrorCode(
               i, _j
            ) if i == LJMErrorCode::from(error_code)
        )
    );
}

#[test]
fn standard_open_read() {
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
        .read_name(open_call, "TEST_INT32".to_string())
        .expect("Expected Value");
    println!("Got: {}", read_value);

    ljm_wrapper
        .write_name(open_call, "TEST_INT32".to_string(), 15)
        .expect("Expected Value");

    let read_value = ljm_wrapper
        .read_name(open_call, "TEST_INT32".to_string())
        .expect("Expected Value");
    println!("Got: {}", read_value);
}
