Defines the LJM Library components

### Structure

At your disposal there is the [`LJMLibrary`] structure.
This has equivalent functions to the LJM C library. To find which
function is equivalent to which, you can [search and enter the LJM function name](./index.html?search=).

To use this library, you must first initialize it,
this is done to purposefully expose the initialisation
as a handleable failure. You can do so by:

```rust
unsafe { LJMLibrary::init(None) }.unwrap();
```

To see this in action, checkout the [examples](https://github.com/bennjii/ljm-rs/tree/master/examples).

The initialisation method takes an override for the underlying
`LJM` library path on your system, if `None` is provided,
a default is selected for you based on the compiled operating system.

### Methods

Methods are provided in the same format as the C library,
you can open a labjack using the following code:

```rust
// handle_id is `Result<i32, LJMError>`, which must be handled..
let handle_id = LJMLibrary::open_jack(DeviceType::ANY, ConnectionType::ANY, "ANY");
```

Every LJM function has the possibility of returning an error,
as is the same behaviour of the underlying library. You should
handle any given error, use of `.unwrap()` is ill advised.

From here, you may pass this handle into any corresponding
function provided by the [`LJMLibrary`] structure.

For example:

```rust
let read_value = LJMLibrary::read_name(handle_id, "TEST_INT32");
```

Which will yield the default test value on a running labjack,
or `0` on a mocked `-2` identifier.

### Extensions

The crate provides first-class support for LabJack's extensions
including [`LJMStream`](https://github.com/bennjii/ljm-rs/blob/master/examples/stream.rs) and [
`LJMLua`](https://github.com/bennjii/ljm-rs/blob/master/examples/lua.rs).

These are gated behind feature flags to minimise compile size and
unnecessary code for those who do not require the extensions.

- [`stream`] Enables streaming (Enabled by default)
- [`lua`] Enables lua modules (Disabled by default)
    - `tokio` Allows for certain functions to be called in a non-blocking manner.

#### Streaming

You may see the example [here](https://github.com/bennjii/ljm-rs/blob/master/examples/stream.rs) for a practical
implementation.
State is stored in a static `OnceLock`, so an invocation of:

```rust
LJMLibrary::stream_start(open_call, scans_per_read, scan_rate, streams);
```

Will then produce the correct buffer size when running

```rust
let value: Vec<f64> = LJMLibrary::stream_read(open_call);
```

The values return by LJMStream are zipped, following the format
of `stream[0], ..., stream[N], stream[0], ...`.

To unzip this, you must keep track of the sensors being read
by LJMStream. This bookkeeping is beyond the scope of `ljmrs`,
but the following function may be of assistance:

<details>
  <summary>Example unzip function</summary>

```rust
/// `unzip(...)`
///
/// Separates the LabJack stream encoding.
/// LabJack values are encoded in the following format.
///
///     [...chunk 1, ...chunk 2, ... , ...chunk n ]
///
/// Where `n` is the number of chunks that will exist in the result,
/// calculable via `scans_per_read` / `num_addresses`.
///
/// Each chunk is given as follows.
///
///     Chunk I: (Representing the Ith value read)
///     +---------- num_addresses --------+
///     |                                 |
///     [ value i, value i, ... , value i ]
///
/// Where the size is given by `num_addresses`, such that for every
/// chunk, there is a reading for each input, laced together.
///
/// Therefore, this function will de-chunk-ify the values
/// returned from LabJack, into the following hash map:
///
///     Sensor1: [ value 1, value 2, ... ]
///     Sensor2: [ value 1, value 2, ... ]
///     ...
///     SensorK: [ value 1, value 2, ... ]
///
fn unzip(sensors: Vec<Sensor>, values: Vec<f64>) -> HashMap<String, Vec<f64>> {
    let chunk_size = values.len() / sensors.len();
    let mut hash = HashMap::new();

    // We aggregate onto the inputs
    for (i, x) in sensors.iter().enumerate() {
        // Separate into individual chunk-lets, them merge into a de-chunked set.
        let de_chunked: Vec<f64> = values
            .chunks(items.len())
            .map(|chunk| chunk[i])
            .collect();

        // Insert the pairing to store this de-chunked set.
        // Must have some form of unique identifier for the sensor.
        hash.insert(x.id, de_chunked);
    }

    hash
}
```

</details>

#### Lua Scripting

You may see the example [here](https://github.com/bennjii/ljm-rs/blob/master/examples/stream.rs) for a practical
implementation.

You are simply required to create the module by passing any `T: ToString` into `LJMLua::new`, like so:

```rust
let module = LJMLua::new(SCRIPT);
```

Then, you may set this module on your designated LabJack like so:

```rust
let debug = true;
let module_set = LJMLibrary::set_module(handle_id, module, debug); // .await;
```

Note, this returns a result for a possible failure, and enabling
the `tokio` feature, will allow you to `.await` this as a future,
otherwise it will block.

You may also note, `debug` is optional. If you do enable it,
you can read `print(...)` statements sent from your `lua` script,
by reading the `LUA_DEBUG_NUM_BYTES` and `LUA_DEBUG_DATA` registers.