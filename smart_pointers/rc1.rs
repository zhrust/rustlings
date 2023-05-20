// rc1.rs
// In this exercise, we want to express the concept of multiple owners via the Rc<T> type.
// This is a model of our solar system - there is a Sun type and multiple Planets.
// The Planets take ownership of the sun, indicating that they revolve around the sun.

// Make this code compile by using the proper Rc primitives to express that the sun has multiple owners.


use std::rc::Rc;

#[derive(Debug)]
struct Sun {}

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hi from {:?}!", self)
    }
}

fn main() {
    let sun = Rc::new(Sun {});
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

    let mercury = Planet::Mercury(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 references
    mercury.details();

    let venus = Planet::Venus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 references
    venus.details();

    let earth = Planet::Earth(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 references
    earth.details();

    let mars = Planet::Mars(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 references
    mars.details();

    let jupiter = Planet::Jupiter(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 references
    jupiter.details();

    // TODO
    //let saturn = Planet::Saturn(Rc::new(Sun {}));
    let saturn = Planet::Saturn(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 references
    saturn.details();

    // TODO
    //let uranus = Planet::Uranus(Rc::new(Sun {}));
    let uranus = Planet::Uranus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 references
    uranus.details();

    // TODO
    //let neptune = Planet::Neptune(Rc::new(Sun {}));
    let neptune = Planet::Neptune(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 9 references
    neptune.details();

    assert_eq!(Rc::strong_count(&sun), 9);


    drop(neptune);
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 references

    drop(uranus);
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 references

    drop(saturn);
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 references

    drop(jupiter);
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 references

    drop(mars);
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 references

    // TODO 同上
    drop(earth);
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 references

    // TODO
    drop(venus);
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 references

    // TODO
    drop(mercury);
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

    assert_eq!(Rc::strong_count(&sun), 1);
}


/*
我们使用了Rc指针来管理Sun实例的所有权。在这个例子中，我们首先使用Rc::new方法创建一个Rc指针来管理Sun实例的所有权，然后将这个指针传递给Planet枚举类型中的每个变量。但是，在将指针传递给枚举变量之后，我们需要确保该指针在Planet变量之前被释放，否则当Planet变量超出作用域时，Rc计数器的值将不会正确地减少，从而导致内存泄漏或意外崩溃。

....

首先创建了一个Rc指针来管理Sun实例的所有权。然后，我们创建了Planet枚举类型中的每个变量，这一次我们使用Rc::clone方法来创建Rc指针的新引用。接着，我们打印出了Rc计数器的值，以检查它是否为我们预期的9个引用。接下来，我们打印了每个Planet变量，以确保它们被正确地创建和使用。最后，我们删除了所有的Planet变量，以便Rc计数器的值可以正确地减少

*/
