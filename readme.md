### `ljmrs`

A rust library which allows you to connect with the LabJack T7 and T8 series through the C/C++ Bindings. This is a rust
abstraction layer for stronger types and safety.

You can install the [crate](https://crates.io/crates/ljmrs) with:

```rust,ignore
cargo add ljmrs
```

Types are provided for LabJack error codes, as a return value for each function.

Documentation is supported around the [wrapper](./ljm/wrapper/struct.LJMWrapper.html). To find equivalent functions of
LJM functions, use [search and enter the LJM function name](./index.html?search=LJM_eReadName).

#### Support

This **does not support** every function yet, you are welcome to create a PR to add any functions you want.

The official documentation from LabJack is
found [here](https://labjack.com/pages/support/software?doc=/software-driver/ljm-users-guide/ljm-users-guide/). `ljm-rs`
simply provides abstraction to the C/C++ library, through the `libloading` crate.

#### Examples

Examples are found in the `/examples` folder. To try an example, run the following:

```bash
cargo run --example <example_name>
```

For example:

```bash
# cargo run --example info
Opened LabJack, got handle: 1
Got IP, 109.61.99.68
```

#### Addendum

Note, running on MacOS with an ARM CPU requires newer versions of LabJack software,
found [here](https://labjack.com/pages/support?doc=/software-driver/installer-downloads/ljm-software-installers-t4-t7-digit/#header-three-ak4ld).
Alternatively, you can use [Rosetta 2](https://support.apple.com/en-us/HT211861) with older software, and the following
command:

```bash
cargo build && arch -x86_64 ./target/x86_64-apple-darwin/debug/ljm-rs
```
