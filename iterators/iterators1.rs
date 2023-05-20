// iterators1.rs
//
//  Make me compile by filling in the `???`s
//
// When performing operations on elements within a collection, iterators are essential.
// This module helps you get familiar with the structure of using an iterator and
// how to go through elements within an iterable collection.
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a hint.


use std::io::Error;

fn main () -> Result<(), Box<dyn std::error::Error>>{
    let my_fav_fruits: Vec<&'static str> = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter();   // TODO: Step 1

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));     // TODO: Step 2
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));     // TODO: Step 3
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), None);     // TODO: Step 4

Ok(())
}

/*

修改的细节如下：

在第 3 行代码中，将 my_fav_fruits 声明为 Vec<&'static str> 类型，表示 vector 中的元素是字符串的引用，并且生命周期是静态的。

在第 5 行代码中，使用 iter() 方法获取 vector 的迭代器，该方法返回的是 std::slice::Iter<'_, &'static str> 类型的迭代器，该类型的迭代器返回的是 &'static str 类型的值的引用。

在第 9-15 行代码中，直接比较迭代器返回的 Option<&'static str> 类型的值和期望值即可。由于 vector 中的元素已经被取走，因此最后一次调用 next() 方法返回的是 None。

在第 17 行代码中，返回一个 Result<(), Box<dyn std::error::Error>> 类型的值，表示该函数运行成功。如果运行失败，则返回一个错误对象。

*/
