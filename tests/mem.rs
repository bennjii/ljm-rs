use ljmrs;
use ljmrs::{LJMError, LJMErrorCode, LJMLibrary};

fn assert_error(error: LJMError, error_code: i32) {
    assert!(matches!(
        error,
        LJMError::ErrorCode(
           i, _j
        ) if i == LJMErrorCode::from(error_code)
    ));
}

#[test]
fn bad_open() {
    if let Err(error) = unsafe { LJMLibrary::init() } {
        debug_assert!(false, "Unable to init LJM. Reason: {:?}", error);
    }

    // Forge a fake handle
    let handle: i32 = -1;
    let result = LJMLibrary::read_name(handle, "AIN0".to_string());

    assert!(result.is_err());

    let error: LJMError = result.err().unwrap();

    assert_error(error, 1224);
}

#[test]
fn fake_write() {
    if let Err(error) = unsafe { LJMLibrary::init() } {
        debug_assert!(false, "Unable to init LJM. Reason: {:?}", error);
    }

    // Forge a fake handle
    let handle: i32 = -1;

    // Forge a pin num. AIN2 with a _RANGE property
    let modbus_range = format!("{}_RANGE", "AIN2");

    let result = LJMLibrary::write_name(handle, modbus_range, 0);

    assert!(result.is_err());

    let error: LJMError = result.err().unwrap();

    assert_error(error, 1224);
}

#[test]
fn use_after_write() {
    if let Err(error) = unsafe { LJMLibrary::init() } {
        debug_assert!(false, "Unable to init LJM. Reason: {:?}", error);
    }

    // Forge a fake handle
    let handle: i32 = -1;

    // Forge a pin num. AIN2 with a _RANGE property
    let modbus_range = format!("{}_RANGE", "AIN2");

    let result = LJMLibrary::write_name(handle, modbus_range, 0);

    assert!(result.is_err());
    let error: LJMError = result.err().unwrap();
    assert_error(error, 1224);
}

// This is a test to intentionally try to break the internal LJM code.
// In an effort to detect any use of internal free's that may break
// when called by rust's automatic `drop` calls, as it may lead to a double free.
#[test]
fn uaw2() {
    if let Err(error) = unsafe { LJMLibrary::init() } {
        debug_assert!(false, "Unable to init LJM. Reason: {:?}", error);
    }

    // Forge a fake handle
    let handle: i32 = -1;

    // Block scope to auto-drop this block.
    {
        let handle_ref = &handle;

        // Forge a pin num. AIN2 with a _RANGE property
        let pin = "AIN2".to_string();
        let modbus_range = format!("{}_RANGE", pin);
        let range = 0;

        if let Err(error) = LJMLibrary::write_name(handle_ref.clone(), modbus_range.clone(), range)
        {
            println!(
                "Unable to write modbus range {} for {}, on {}. Reason: {:?}",
                range, modbus_range, handle_ref, error
            );

            // assert!(result.is_err());
            // let error: LJMError = result.err().unwrap();
            assert_error(error, 1224);
        }
    }

    assert!(true);
}
