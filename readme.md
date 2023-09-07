### `ljmrs`

A rust library which allows you to connect with the labjack T7 and T8 series through the C/C++ Bindings with a rust abstraction layer for stronger types and safety.

As Mac does not have an ARM LJM library yet, you can run with:

```
cargo build && arch -x86_64 target/x86_64-apple-darwin/debug/ljm-rs
```
