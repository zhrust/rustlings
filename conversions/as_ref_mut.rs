// AsRef and AsMut allow for cheap reference-to-reference conversions.
// Read more about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html
// and https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a hint.


//use std::ops::Mul;

// Obtain the number of bytes (not characters) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
/* 
fn byte_counter<T>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}
*/
fn byte_counter<T: AsRef<[u8]>>(arg: T) -> usize {
    arg.as_ref().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
/* 
fn char_counter<T>(arg: T) -> usize {
    arg.as_ref().chars().count()
}
 */
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using as_mut().
// TODO: Add the appropriate trait bound.
/* 
fn num_sq<T: std::ops::Mul<Output = T> + std::marker::Sized>(arg: &mut T) {
    *arg = std::mem::take(arg) * std::mem::take(arg);
}
fn num_sq<T>(arg: &mut T) {
    // TODO: Implement the function body.
    ???
    
}

fn num_sq<T>(arg: &mut T)
    where T: std::ops::Mul<Output = T> + std::ops::MulAssign + Copy
{
    let square = *arg * *arg;
    *arg *= *arg;
    *arg = square;
}

没错, 但是编译有更多报错了:
error[E0277]: cannot multiply `Box<u32>` by `Box<u32>`
  --> exercises/conversions/as_ref_mut.rs:86:16
   |
86 |         num_sq(&mut num);
   |         ------ ^^^^^^^^ no implementation for `Box<u32> * Box<u32>`
   |         |
   |         required by a bound introduced by this call
   |
   = help: the trait `Mul` is not implemented for `Box<u32>`
note: required by a bound in `num_sq`
  --> exercises/conversions/as_ref_mut.rs:47:14
   |
46 | fn num_sq<T>(arg: &mut T)
   |    ------ required by a bound in this
47 |     where T: std::ops::Mul<Output = T> + std::ops::MulAssign + Copy
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `num_sq`

error[E0277]: cannot multiply-assign `Box<u32>` by `Box<u32>`
  --> exercises/conversions/as_ref_mut.rs:86:16
   |
86 |         num_sq(&mut num);
   |         ------ ^^^^^^^^ no implementation for `Box<u32> *= Box<u32>`
   |         |
   |         required by a bound introduced by this call
   |
   = help: the trait `MulAssign` is not implemented for `Box<u32>`
note: required by a bound in `num_sq`
  --> exercises/conversions/as_ref_mut.rs:47:42
   |
46 | fn num_sq<T>(arg: &mut T)
   |    ------ required by a bound in this
47 |     where T: std::ops::Mul<Output = T> + std::ops::MulAssign + Copy
   |                                          ^^^^^^^^^^^^^^^^^^^ required by this bound in `num_sq`

error[E0277]: the trait bound `Box<u32>: Copy` is not satisfied
  --> exercises/conversions/as_ref_mut.rs:86:16
   |
86 |         num_sq(&mut num);
   |         ------ ^^^^^^^^ the trait `Copy` is not implemented for `Box<u32>`
   |         |
   |         required by a bound introduced by this call
   |
note: required by a bound in `num_sq`
  --> exercises/conversions/as_ref_mut.rs:47:64
   |
46 | fn num_sq<T>(arg: &mut T)
   |    ------ required by a bound in this
47 |     where T: std::ops::Mul<Output = T> + std::ops::MulAssign + Copy
   |                                                                ^^^^ required by this bound in `num_sq`

error: aborting due to 3 previous errors

 */


fn num_sq<T>(arg: &mut T)
where
    T: std::ops::Deref<Target = u32> + std::ops::DerefMut,
{
    let square = **arg * **arg;
    **arg = square;
}

/*
这些报错是因为 Box<T> 类型没有实现 std::ops::Mul 和 std::ops::MulAssign trait。你可以使用 Deref trait 来解引用 Box<T> 类型的值，然后再对值进行乘法运算和赋值操作。

上面是修改后的代码：

在这个版本的代码中，T 类型必须实现 std::ops::Deref 和 std::ops::DerefMut trait，这样就可以使用解引用操作符 * 来获取 Box<T> 类型内部的值，并进行乘法运算和赋值操作了。

值得注意的是，在这个版本的代码中，我们不需要显式地实现 Copy trait，因为 u32 类型已经实现了 Copy trait，而 T 类型需要实现 Deref<Target = u32> trait，所以也就具有了 Copy trait 的能力。

*/


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}



/*
fn num_sq<T: std::ops::Mul<T, Output=T> + std::ops::Copy>(arg: &mut T) {
    *arg = (*arg) * (*arg);
}
fn num_sq<T: std::ops::Mul<Output=T> + Copy>(arg: &mut T) {
    *arg = *arg * *arg;
}

fn num_sq<T: std::ops::Mul<Output=T> + Copy>(arg: &mut T) {
    *arg = *arg * *arg;
}
fn num_sq<T: std::ops::Mul<Output=T> + Clone>(arg: &mut T) {
    *arg = arg.clone() * arg.clone();
}



fn num_sq<T: Mul<Output = T> + Copy>(arg: &mut T) {
    *arg = *arg * *arg;
}

impl Mul for Box<u32> {
    type Output = Box<u32>;

    fn mul(self, other: Box<u32>) -> Box<u32> {
        Box::new(*self * *other)
    }
}
*/

