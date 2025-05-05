# funcall
A lightweight Rust library that turns functions into JSON-callable tools with zero boilerplate using a simple `#[funcall]` macro.

## Overview

`funcall` is a lightweight Rust framework that enables dynamic function calling through JSON interfaces. It provides macros to automatically wrap Rust functions and expose them as callable tools with JSON serialization/deserialization.

## Features

- **Automatic function wrapping**: Convert regular Rust functions into JSON-callable tools
- **Type-safe argument handling**: Supports primitive types, `Option`, `Vec`, and any `Deserialize` types
- **Dynamic invocation**: Call functions by name at runtime
- **Minimal boilerplate**: Simple attribute macro syntax

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
funcall = "0.2.0"
serde_json = "1.0"
serde = { version = "1.0", features = ["derive"] }
```

## Usage

### 1. Define Functions

```rust
use funcall::funcall;
use serde::Deserialize;

#[funcall]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[funcall]
fn max(a: i32, b: i32, c: i32) -> i32 {
    *[a, b, c].iter().max().unwrap()
}

#[funcall]
fn greet(name: Option<String>, excited: bool) -> String {
    match name {
        Some(n) if excited => format!("Hello, {}!!!", n),
        Some(n) => format!("Hello, {}.", n),
        None => "Hello!".to_string(),
    }
}

#[funcall]
fn sum(numbers: Vec<i32>) -> i32 {
    numbers.iter().sum()
}

#[derive(Deserialize)]
struct User {
    name: String,
    age: u8,
}

#[funcall]
fn describe_user(user: User) -> String {
    format!("{} is {} years old", user.name, user.age)
}
```

### 2. Register and Call Functions

```rust
use funcall::tools;
use serde_json::json;

let tools = tools![add, max, greet, sum, describe_user];

// Basic math operations
let result = tools["add"](&json!([2, 3]));
println!("2 + 3 = {}", result); // 5

let result = tools["max"](&json!([1, 5, 3]));
println!("max(1, 5, 3) = {}", result); // 5

// Flexible greeting function
let result = tools["greet"](&json!(["Morris", true]));
println!("{}", result); // "Hello, Morris!!!"

let result = tools["greet"](&json!([null, true]));
println!("{}", result); // "Hello!"

// Working with collections
let result = tools["sum"](&json!([[1, 2, 3, 4]]));
println!("sum([1,2,3,4]) = {}", result); // 10

// Custom struct handling
let result = tools["describe_user"](&json!([{
    "name": "Morris",
    "age": 30
}]));
println!("{}", result); // "Morris is 30 years old"

// Named arguments
let res = tools["greet"](&json!({
    "name": "Morris",
    "excited": true
}));
println!("result = {}", res); // Hello, Morris!!!
```

## Supported Argument Types

| Type       | JSON Representation | Notes                      |
|------------|---------------------|----------------------------|
| `i32`      | Number              | Converted from i64         |
| `f64`      | Number              |                            |
| `bool`     | Boolean             |                            |
| `String`   | String              |                            |
| `Option<T>`| Any or null         | Null becomes None          |
| `Vec<T>`   | Array               | T must be deserializable   |
| Custom     | Object/Array/etc    | Must implement Deserialize |

## Error Handling

- Invalid JSON input will panic
- Wrong number of arguments will panic
- Type mismatches will panic

For production use, consider wrapping calls in error handling.

## Limitations

- Function names must be valid Rust identifiers
- All arguments must be positional (no named arguments)
- Return types must be serializable to JSON

## License

MIT

## Contribution

Contributions are welcome! Please open issues or pull requests on GitHub.
