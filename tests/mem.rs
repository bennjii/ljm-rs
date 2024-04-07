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
fn fake_write() {
    let ljm_wrapper = unsafe { LJMWrapper::init(None) }.unwrap();

    // Forge a fake handle
    let handle: i32 = -1;

    // Forge a pin num. AIN2 with a _RANGE property
    let modbus_range = format!("{}_RANGE", "AIN2");

    let result = ljm_wrapper.write_name(handle, modbus_range, 0);

    assert!(result.is_err());

    let error: LJMError = result.err().unwrap();

    assert_error(error, 1224);
}

#[test]
fn use_after_write() {
    let ljm_wrapper = unsafe { LJMWrapper::init(None) }.unwrap();

    // Forge a fake handle
    let handle: i32 = -1;

    // Forge a pin num. AIN2 with a _RANGE property
    let modbus_range = format!("{}_RANGE", "AIN2");

    let result = ljm_wrapper.write_name(handle, modbus_range, 0);

    assert!(result.is_err());
    let error: LJMError = result.err().unwrap();
    assert_error(error, 1224);
}
