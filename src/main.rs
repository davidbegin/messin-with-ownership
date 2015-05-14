#![feature(convert)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod ui;
mod basic;
mod learn_rust_ownership;

fn main() {
    ui::title();
    // basic::sample_1();
    learn_rust_ownership::the_hard_way()
}
