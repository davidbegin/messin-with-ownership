#![allow(dead_code)]
#![allow(unused_variables)]

mod ui;

fn main() {
    ui::title();

    // let x: i32; // x goes into scope
    //
    // {
    //     let y = &5;
    //     let f = Foo { x: y };
    //     x = &f.x;
    // }


} // x leaves scope

// struct Foo<'a> {
//     x: &'a i32
// }
//
// fn lifetime_chaos() {
//     let y = &15;
//     let f = Foo { x: y };
//
//     println!("{}", f.x);
// }

// struct Foo has a lifetime a
// struct Foo<'a> {
//     x: &'a i32,
// }

// struct Foo {
//     x: i32,
// }

fn chaos() {
    // ownership, borrowing, and lifetimes

    // This is all about memory
    // something I never worry about

    // So the deeper question is:
    // How can I start worrying about memory?

    // So first we declare a variable on the Heap
    let mut x = 5;
    println!("Heres x: {}", x);
    add_one(&mut x);
    // this won't compile, because x is moved
    // can I change this fact?
    println!("Heres x: {}", x)
}

// the add one function takes a Heap allocated i32
// and modifies it to be +1
//
// why do we need a *num here,
// isn't the * deferencing num,
// but its not borrowed, or since its box
// we need to?!?
fn add_one<'a>(num: &'a mut i32) {
    *num += 1;
}
