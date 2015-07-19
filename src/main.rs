#![feature(convert)]
#![allow(dead_code, unused_variables, unused_must_use, unused_mut)]

extern crate type_printer;
mod ui;
mod basic;
mod learn_rust_ownership;
mod iterators_borrowing_and_other_stuff;
mod this_is_how_we_do_it;
mod structs_and_borrowing;

fn main() {
    ui::title();
    // basic::sample_1();
    // learn_rust_ownership::the_hard_way()
    // iterators_borrowing_and_other_stuff::land_of_confusion();
    // this_is_how_we_do_it::this_it_how_we_do_it();
    // mut_ref_what();
    structs_and_borrowing::an_exploration();
}

fn mut_ref_what() {
    let mut x: i32 = 5;

    // let y = &5;
    // won't compile because although x is mutable
    // when we take a reference it is immutable
    // unless we say so
    // y += 1;

    // let mut z: i32 = &mut x;
    // also won't compile
    // expected `i32`,
    // found `&mut i32`
    // (expected i32,
    // found &-ptr)
    // z + 10;

    let mut x: i32 = 5;
    let a = x;
    type_printer::print_type_of(&a); // i32

    let mut x: i32 = 5;
    let b = &x;
    type_printer::print_type_of(&b); // &'static i32

    let mut x: i32 = 5;
    let c = &mut x;
    type_printer::print_type_of(&c); // &'static mut i32

    let mut x: i32 = 5;
    let mut d = &mut x;
    type_printer::print_type_of(&d); // &'static mut i32'
}
