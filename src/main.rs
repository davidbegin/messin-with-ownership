#![feature(convert)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod ui;
mod basic;
mod learn_rust_ownership;
mod iterators_borrowing_and_other_stuff;

fn main() {
    ui::title();
    // basic::sample_1();
    // learn_rust_ownership::the_hard_way()
    iterators_borrowing_and_other_stuff::land_of_confusion();
}
