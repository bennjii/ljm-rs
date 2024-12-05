extern crate ljmrs;

use ljmrs::{LJMLibrary, LJMLua};

const SCRIPT: &str = include_str!("example.lua");

fn init() -> i32 {
    unsafe { LJMLibrary::init() }.unwrap();

    LJMLibrary::open_jack(
        ljmrs::DeviceType::ANY,
        ljmrs::ConnectionType::ANY,
        "-2".to_string(),
    )
    .expect("Could not open DEMO LabJack")
}

#[cfg(feature = "tokio")]
#[tokio::main]
async fn main() {
    let open_call = init();

    let module = LJMLua::new(SCRIPT);
    println!("Setting LUA module of size: {}", module.size());

    LJMLibrary::set_module(open_call, module, true)
        .await
        .unwrap();
    println!("Module set!");
}

#[cfg(not(feature = "tokio"))]
fn main() {
    let open_call = init();
    let module = LJMLua::new(SCRIPT);
    println!("Setting LUA module of size: {}", module.size());

    LJMLibrary::set_module(open_call, module, true).unwrap();
    println!("Module set!");
}
