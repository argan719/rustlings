#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);   // 밑에서 data를 또 사용해야하기 때문에 여기서는 소유권 이동이 일어나면 안됨. 따라서 참조 사용

    string_uppercase(data);  // 여기서는 상관없음.
}
