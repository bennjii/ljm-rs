fn load() {
    let now = Instant::now();

    let ljm_wrapper = unsafe { LJMWrapper::init() };

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();

    let (addr, typ) = ljm_wrapper.name_to_address("AIN0".to_string());

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();

    ljm_wrapper.digital_write_to_address("AIN0".to_string(), "AIN0_RANGE".to_string(), 15_u32);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("Function result: {}:{}", addr, typ);
}
