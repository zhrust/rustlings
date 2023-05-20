// lifetimes2.rs
//
// So if the compiler is just validating the references passed
// to the annotated parameters and the return type, what do
// we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a hint.


use std::borrow::Cow;

//fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
fn longest<'a>(x: Cow<'a, str>, y: Cow<'a, str>) -> Cow<'a, str> {
//fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn main() {
    let string1 = "long string is long".to_string();
    let string2 = "xyz".to_string();
    let result = longest(Cow::Borrowed(&string1), Cow::Borrowed(&string2));
    println!("The longest string is '{}'", result);
}

/* 
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        //result = longest(string1.as_str(), string2.as_str());
        result = longest(string1.as_ref().as_str(), string2.as_ref().as_str());
        result = longest(<String as AsRef<T>>::as_ref(&string1).as_str(), string2.as_ref().as_str());
        
        //result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is '{}'", result);
}
 */
/*
使用 std::borrow::Cow 类型来避免所有权和生命周期问题。Cow 类型是一个叫做 Copy On Write 的技术，它可以在必要时进行复制，以避免引用的所有权和生命周期问题。在 Rust 中，Cow 类型通常用于函数的输入参数或者输出参数，它可以使得代码更加灵活和易于处理不同的场景。

*/
