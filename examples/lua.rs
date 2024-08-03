extern crate ljmrs;

use ljmrs::{LJMWrapper, LuaModule};

const SCRIPT: &str = r#"
local ramval = 0
MB.W(46180, 0, ramval)
local loop0 = 0
local loop1 = 1
local loop2 = 2

-- Setup an interval to control loop execution speed. Update every second
LJ.IntervalConfig(0,1000)
while true do
  if LJ.CheckInterval(0) then
    ramval = MB.R(46180, 0)

    if ramval == loop0 then
      print("using loop0")
    end

    if ramval == loop1 then
      print("using loop1")
    end

    if ramval == loop2 then
      print("using loop2")
    end

  end
end
"#;

fn init() -> i32 {
    unsafe { LJMWrapper::init(None) }.unwrap();

    LJMWrapper::open_jack(
        ljmrs::DeviceType::ANY,
        ljmrs::ConnectionType::ANY,
        "-2".to_string(),
    ).expect("Could not open DEMO LabJack")
}

#[cfg(feature = "tokio")]
#[tokio::main]
async fn main() {
    let open_call = init();

    let module = LuaModule::new(SCRIPT);
    println!("Setting LUA module of size: {}", module.size());

    LJMWrapper::set_module(open_call, module).await.unwrap();
    println!("Module set!");
}

#[cfg(not(feature = "tokio"))]
fn main() {
    let open_call = init();
    let module = LuaModule::new(SCRIPT);
    println!("Setting LUA module of size: {}", module.size());

    LJMWrapper::set_module(open_call, module).unwrap();
    println!("Module set!");
}
