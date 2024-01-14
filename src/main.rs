use std::fmt::Write as _;

pub fn long_say_hello(name: &str) -> String {
    let mut result = "Hello ".to_owned();
    result.push_str(name);
    result.push('!');
    result
}

pub fn format_say_hello(name: &str) -> String {
    format!("Hello {name}!")
}

pub fn write_say_hello(name: &str) -> String {
    let mut buffer = String::with_capacity(name.len());
    write!(&mut buffer, "Hello {name}!").unwrap();
    buffer
}

fn main() {
    println!("{}", long_say_hello("Amy"));
    println!("{}", format_say_hello("Amy"));
    println!("{}", write_say_hello("Amy"));
}
