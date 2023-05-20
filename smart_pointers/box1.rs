// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This becomes problematic
// for recursive types, where a value can have as part of itself another value of the same type.
// To get around the issue, we can use a `Box` - a smart pointer used to store data on the heap,
// which also allows us to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a data structure
// frequently found in functional programming languages. Each item in a cons list contains two
// elements: the value of the current item and the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `todo!()`
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}
/*
pub enum List {
    Cons(i32, List),
    Nil,
}


在 Rust 中，枚举可以包含多个变量，其中一个变量可以是当前枚举类型本身。例如，List 枚举类型中的 Cons 变量将包含一个 i32 类型的整数和另一个 List 类型的变量，这个变量实际上是当前的 List 枚举类型。

这样的定义看起来很方便，因为它让我们可以创建一个包含任意数量元素的链表，但是这也会导致递归类型错误。在这种情况下，编译器无法确定枚举类型的大小，因为它包含了自己，这是无限的循环，直到栈溢出。

通过使用 Box 或其他智能指针，我们将其转换为间接递归类型。在 Rust 中，指针有固定的大小，所以编译器可以正常地处理这样的类型。

因此，在 Rust 中，如果你想要一个递归类型，你必须使用指针类型进行包装，否则编译器将报告递归类型错误，并拒绝编译你的代码。
*/
fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    //todo!()
    List::Nil
}

pub fn create_non_empty_list() -> List {
    //todo!()
    List::Cons(1, Box::new(List::Nil))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
