use std::cell::RefCell;
use std::rc::Rc;

// Box, Rc, and RefCell

fn main() {
    println!("Memory management Demo in rust!");

    //ownership
    let s1 = String::from("Ownership example");
    let s2 = s1; // ownership moved
    println!("Ownership transfered: {}", s2);

    let s3 = String::from("Borrowing Example");
    borrow_demo(&s3);
    println!("After borrow: {s3}");

    let mut s4 = String::from("hello");
    mutate_demo(&mut s4);
    println!("After mutation: {s4}");

    // Lifetimes
    let result;
    let a = String::from("abcd");
    {
        let b = String::from("xyz");
        result = longest(&a, &b);
        println!("Longest string: {result}");
    }

    let boxed = Box::new(42);
    println!("Boxed value: {boxed}");

    let rc_val = Rc::new(String::from("Shared"));
    let rc_clone = Rc::clone(&rc_val);
    println!("Rc values: {}, {}", rc_val, rc_clone);
    println!("Ref Count: {}", Rc::strong_count(&rc_val));

    // RefCell (interior mutablilty)
    let cell = RefCell::new(100);
    *cell.borrow_mut() += 50;
    println!("Refcell value: {}", cell.borrow());
}

fn borrow_demo(s4: &String) {
    println!("Boorwed data: {s4}");
}

fn mutate_demo(data: &mut String) {
    data.push_str("World");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
