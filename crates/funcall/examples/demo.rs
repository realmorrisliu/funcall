use funcall::{funcall, tools};
use serde::Deserialize;
use serde_json::json;

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

fn main() {
    let tools = tools![add, max, greet, sum, describe_user];
    let res = tools["add"](&json!([2, 3]));
    println!("result = {}", res); // 5

    let res = tools["max"](&json!([1, 5, 3]));
    println!("result = {}", res); // 5

    let res = tools["greet"](&json!(["Morris", true]));
    println!("result = {}", res);

    let res = tools["greet"](&json!([null, true]));
    println!("result = {}", res);

    let res = tools["sum"](&json!([[1, 2, 3, 4]]));
    println!("result = {}", res);

    let res = tools["describe_user"](&json!([{
        "name": "Morris",
        "age": 30
    }]));
    println!("result = {}", res);
}
