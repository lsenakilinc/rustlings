#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}
//since string is not copy type it shouldn't take ownership so that we can be able to use it later on

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();
//
    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}
