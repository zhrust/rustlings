// iterators4.rs
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a hint.


pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    (1..=num).fold(1, |acc, n| acc * n)
}
/*
每个阶乘的值可以通过对前一个阶乘值乘以当前数字来计算。
这个计算可以通过调用fold方法来实现。
在fold方法的闭包中，
第一个参数acc代表计算的当前结果，
而第二个参数n代表当前的数字。
为了解决num为0或1的特殊情况，可以将起始值设置为1，然后迭代1到num。最后返回累积的结果即可。
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
