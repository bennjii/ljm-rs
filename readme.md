### `ljmrs`

A rust library which allows you to connect with the labjack T7 and T8 series through the C/C++ Bindings with a rust abstraction layer for stronger types and safety.

> Build Target is not specific, but does not support specific ARM64 archs (M1,M2), due to LJM restrictions.
> Therefore on MacOS you can run with the following:
> ```
> cargo build && arch -x86_64 target/x86_64-apple-darwin/debug/ljm-rs
> ```
> This requires [Rosetta 2](https://support.apple.com/en-us/HT211861).

You can install the [crate](https://crates.io/crates/ljmrs) with:
```
cargo add ljmrs
```

Happy Coding!
