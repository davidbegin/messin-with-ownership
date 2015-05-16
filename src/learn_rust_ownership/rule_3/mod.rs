pub fn experiments() {
    let rule_3: &str = "
        Blocks produce a value which goes up one level of scope
    ";

    println!("\nRule 3: {}\n", rule_3);

    // YESSSS, finally something that makes sense!

    // I mean
    let x = {{{{{Some(1)}}}}};

    // and I got a nice explicit return a semi-colon offical!
    foo_the_c_way();
}

fn foo_the_c_way() -> i32 {

    return 1;
}
