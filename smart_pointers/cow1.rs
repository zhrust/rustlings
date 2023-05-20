// cow1.rs

// This exercise explores the Cow, or Clone-On-Write type.
// Cow is a clone-on-write smart pointer.
// It can enclose and provide immutable access to borrowed data, and clone the data lazily when mutation or ownership is required.
// The type is designed to work with general borrowed data via the Borrow trait.


//use std::borrow::Borrow;
use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}
/*
这段代码使用了很多生命周期声明来确保所有的借用都是有效的，并且避免了悬垂指针和内存安全问题。

在这里，Cow 的生命周期被分为两部分，'a 和 'b。'b 生命周期是 Cow 内部的引用的生命周期，它标识了 Cow 所借用的数据的生命周期。'a 生命周期是 input 借用的生命周期，它表示函数的返回值的生命周期，即 &'a mut Cow<'b, [i32]>。通过声明这些生命周期，编译器可以在编译时检查代码，确保它们被正确使用。

此外，&'a mut Cow<'b, [i32]> 表示 input 是一个可变的借用，因此函数可以修改借用的数据。Cow::from(&slice[..]) 和 Cow::from(slice) 创建了一个 Cow<'b, [i32]>，&'a mut Cow<'b, [i32]> 表示可变的借用，因此可以对其进行修改，例如在函数中调用 input.to_mut()。

总之，这些生命周期声明的作用是帮助编译器检查代码并保证内存安全，避免悬垂指针和其他常见的问题。

'b 生命周期用于限定 Cow 的内部引用的生命周期。在 Cow 中，它包含一个 enum，其中可能是一个指向借用数据的引用(&'b [i32])，也可能是一个指向拥有数据的 Vec<i32>。因此，'b 生命周期限制了 Cow 借用数据的生命周期，以确保不会出现悬垂指针或其它类似的错误。

具体而言，在函数签名 fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> 中，'a 生命周期表示返回值的生命周期，而 'b 生命周期表示 Cow 所借用的数据的生命周期。这使得函数能够在不出现悬垂指针或其他类似问题的情况下修改借用的数据。因此，在函数中，当需要对借用数据进行修改时，需要使用 to_mut() 方法来获取一个 &mut [i32] 引用，然后才能进行修改。


*/

fn main() {
    // No clone occurs because `input` doesn't need to be mutated.
    let slice = [0, 1, 2];
    let mut input = Cow::from(&slice[..]);
    match abs_all(&mut input) {
        Cow::Borrowed(_) => println!("I borrowed the slice!"),
        _ => panic!("expected borrowed value"),
    }

    // Clone occurs because `input` needs to be mutated.
    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    match abs_all(&mut input) {
        Cow::Owned(_) => println!("I modified the slice and now own it!"),
        _ => panic!("expected owned value"),
    }

    // No clone occurs because `input` is already owned.
    let slice = vec![-1, 0, 1];
    let mut input = Cow::from(slice);
    let result = abs_all(&mut input);
    match abs_all(&mut input) {
        Cow::Owned(_) => println!("I own this slice!"),
        _ => panic!("expected owned value"),
    }
/* 
input已经是所有权所有者，因此无需克隆或借用。因此，在第三个 match 分支中，应该匹配 Cow::Owned(_)，而不是 Cow::Borrowed(_)。因此，将 TODO 行更改为以下内容应该解决问题：


    match abs_all(&mut input) {
        // TODO
        Cow::Borrowed(_) => println!("I own this slice!"),
        _ => panic!("expected borrowed value"),
    } 
 */
}
