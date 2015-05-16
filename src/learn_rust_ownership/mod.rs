// http://chrismorgan.info/blog/rust-ownership-the-hard-way.html

extern crate type_printer;

mod rule_1;
mod rule_2;
mod rule_3;
mod rule_4;

pub fn the_hard_way() {
    rule_1::experiments();
    rule_2::experiments();
    rule_3::experiments();
    rule_4::experiments();
}
