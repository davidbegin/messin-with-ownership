#![feature(convert)]
#![allow(dead_code, unused_variables, unused_must_use)]

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
