// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a hint.



fn main() {
    //let mut shopping_list: Vec<?> = Vec::new();
    let mut shopping_list: Vec<String> = Vec::new();
    shopping_list.push("milk".to_string());
}


/*

let mut shopping_list: Vec<&'static str> = Vec::new();
shopping_list.push("milk");



Vec<&'static str> 表示一个元素类型为静态字符串引用（&'static str）的向量（Vec）。 'static 生命周期是 Rust 中的一种特殊生命周期，它表示该引用的生命周期是静态的，即在程序运行期间始终存在，不会被释放。

在这种方案中，我们将字符串字面量 "milk" 的类型显式指定为 &'static str，这意味着该字符串的生命周期是静态的，即该字符串存储在程序的常量区域，不会被释放。因此，在将其插入 shopping_list 向量时，我们不必担心其生命周期的问题，因为它们始终存在于程序的常量区域，不会被释放。

需要注意的是，在这种方案中，我们不能修改向量中的字符串，因为它们是不可变的。如果需要可变的字符串，则应该使用 String 类型而不是字符串引用类型。

*/
