// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

fn main() {
    let data = "Rust is great!".to_string();

    //get_char(data);
    //string_uppercase(&data);

    let _last = get_char(&data);
    string_uppercase(_last.to_string());
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    //data = &data.to_uppercase();
    data = data.to_uppercase();

    println!("{}", data);
}
