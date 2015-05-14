extern crate type_printer;

pub fn sample_1() {
    let mut v = vec![];
    v.push("Hello".to_string());

    // corral that x, and don't let it loose!
    {
        let x = &v[0];
        type_printer::print_type_of(x);
    }

    // just pass it to a function as borrowed!
    young_programmer_print_that_type(&v[0]);

    {
      let x = (&v[0].as_str());
      type_printer::print_type_of(&&&&&&&x);
      // => &'static &'static &'static &'static &'static &'static &'static
    }

    v.push("world".to_string());
    println!("{:?}", v);
}

fn young_programmer_print_that_type(strang: &String) {
    type_printer::print_type_of(strang);
}
