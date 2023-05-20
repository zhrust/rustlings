// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut x = 100;
    //let y = &mut x;
    //let z = &mut x;
    //*y += 100;
    //*z += 1000;

    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 1000;

    //     使用中间对象, 完成所有权一次借用
    //let y = &mut x;
    //let z = &mut x;
    //let temp = &mut *y;
    //*temp += 100;
    //*z += 1000;
    //y = temp;

    /* 绕开始所有权问题...
        let mut add = vec![];
        add.push(100);
        add.push(1000);
        for n in add {
            x += n;
        }

    可以观察到 fold 计算过程的形式...
        let result = vec.iter().fold(x, |acc, &n| {
            let new_acc = acc + n;
            println!("acc = {}, n = {}, new_acc = {}", acc, n, new_acc);
            new_acc
        });
        */

    assert_eq!(x, 1200);
}
