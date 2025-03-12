# yyjson-rs

A Rust wrapper for [yyjson](https://github.com/ibireme/yyjson), a
high-performance JSON library written in ANSI C.

## Features

- Fast JSON parsing and writing using yyjson
- Flexible memory management
- DOM-style API for JSON navigation
- Full JSON spec support
- Non-standard JSON support
  - Big integers
  - Inf/Nan numbers
  - C-style comments
  - Trailing commas

## Requirements

- Rust 1.56+ (may vary, check actual minimum supported version)

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
yyjson-rs = "0.1.0"  # Check crates.io for latest version
```

## Usage

### Parsing JSON

```rust
use yyjson_rs::DocContext;

fn main() -> anyhow::Result<()> {
    let json = r#"
    {
        "name": "Alice",
        "age": 30,
        "scores": [95.5, 89.2, 92.8],
        "metadata": {
            "active": true,
            "tags": ["rust", "c", "json"]
        }
    }"#;

    let ctx = DocContext::default();
    let doc = ctx.parse(json.as_bytes())?;
    let root = doc.root();

    // Access primitive values
    let name: Option<&str> = root.at_key("name").and_then(|v| v.str());
    let age: Option<u64> = root.at_key("age").and_then(|v| v.u64());

    // Navigate nested structures
    let scores = root.at_key("scores").and_then(|v| v.list()).unwrap();
    let first_score = scores.get(0).and_then(|v| v.f64()).unwrap();

    let active: Option<bool> = root
        .at_key("metadata")
        .and_then(|m| m.at_key("active"))
        .unwrap()
        .bool();
    Ok(())
}
```

Iterate over lists and objects:

```rust
    // Array iteration
    let scores = root.at_key("scores").and_then(|v| v.list()).unwrap();
    for score in scores.iter() {
        println!("Score: {}", score.f64().unwrap());
    }

    // Object iteration
    let metadata = root.at_key("metadata").and_then(|v| v.obj()).unwrap();
    for (key, value) in metadata.iter() {
        println!("{}: {}", key, value);
    }
```

### Writing JSON

```rust
use yyjson_rs::{
    BasicAllocProvider, DocContext, ReadOptions, WriteOptions, Writer, YyjsonAllocProvider,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse doc
    let json = r#"{"name": "John", /* age is invalid */ "age": NaN,}"#;
    let read_opts = ReadOptions {
        allow_trailing_commas: true,
        allow_comments: true,
        allow_inf_and_nan: true,
        ..Default::default()
    };
    let doc_context = DocContext::new(BasicAllocProvider::default(), read_opts);
    let doc = doc_context.parse(json.as_bytes())?;

    // write
    let alloc_provider = BasicAllocProvider::default();
    let allocator = alloc_provider.get_allocator();
    let write_opts = WriteOptions {
        pretty: true,
        allow_inf_and_nan: true,
        ..Default::default()
    };
    let mut writer = Writer::new(allocator, Some(&write_opts));
    let output = doc.write(&mut writer)?;
    println!("{}", output.as_str());

    Ok(())
}
```

## Memory Allocation Strategies

The library supports multiple allocation strategies:

- `BasicAllocProvider`: Default, libc allocator
- `PoolAllocProvider`: Pre-allocates a fixed-length buffer, suitable for single
  use per JSON document rather than across multiple JSON documents
- `DynamicAllocProvider`: Similar to pool allocator, however, when there is not
  enough memory, it dynamically requests more memory using malloc and frees all
  allocations when it is destroyed

## Parsing Options

For non-standard JSON, you can customize parsing with `ReadOptions`:

- `allow_comments`: allow c-style single line and multiple line comments
- `allow_trailing_commas`: allow trailing comma at end of an object or array
- `stop_when_done`: if there is additional content after a JSON document, such
  as a newline, stop once parsing of the JSON document is done rather than
  erroring out
- `allow_inf_and_nan`: Allow inf/nan values
- `bignums_as_raw_strings`: read big numbers that cannot be represented as `u64`
  and `i64`, and floats that cannot be represented by finite `f64` as raw
  strings

## Writing Options

Control JSON output with `WriteOptions`:

- `pretty`: Write JSON pretty with 4 space indent
- `pretty_with_two_spaces`: Write JSON pretty with 2 space indent, overrides
  `pretty`
- `escape_unicode`: escape unicode as `uXXXX`, make output ASCII only
- `escape_slashes`: escape '/' as '\/'.
- `allow_inf_and_nan`: (non-standard) write inf and nan number as 'Infinity' and
  'NaN' literal.
- `inf_and_nan_as_null`: write inf and nan number as null literal. Overrides
  `allow_inf_and_nan`
- `add_newline_at_end`: adds newline character at the end of JSON, helpful for
  NDJSON

## Performance

This library provides a thin wrapper around yyjson, maintaining the original
library's high-performance.

## Safety

While using FFI internally, the public API ensures:

- Lifetime-bound document access
- Null-pointer checks for all C interactions
- Type checking before value access
- Memory safety through RAII patterns

## License

MIT

## Contributing

Contributions are welcome! Please submit pull requests or open issues on the
repository.

## Acknowledgments

- [yyjson](https://github.com/ibireme/yyjson) - The underlying high-performance
  JSON library
