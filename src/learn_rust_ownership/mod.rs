// mod rule_1;
extern crate type_printer;
mod rule_1;

// http://chrismorgan.info/blog/rust-ownership-the-hard-way.html

pub fn the_hard_way() {
    rule_1::experiments();
    // rule_1();
    // rule_2();
    // rule_3();
    // rule_4();
}

fn rule_2() {
    let rule_2: &str = "
        When an object passes out of scope,
        it is destroyed and is no longer usable.
    ";

    println!("Rule 2: {}", rule_2);

    {
        let a = 10;
    }

    // oh no no no
    // println!("{}", a);

    // Time for some Rule 2 practice

    {
        let pattern = "subject";
        // println!("{}", pattern);
    }

    let pattern = "subject";
    // will not compile, because local_var is only in the scope of its branch
    // let result  = match pattern {
    //     local_var @ "subject" => local_var,
    //     _         => local_var
    // };

    // Not sure if this is the same example as in the blog article

    let subject = [1019, 3239];
    for pattern in subject.iter() {
        println!("ah {}, a fine reference!", pattern);
        // pattern is available here
    }
    // and of course this won't compile
    // println!("{}", pattern)
}

fn rule_3() {
    let rule_3: &str = "
        Blocks produce a value which goes up one level of scope
    ";

    // YESSSS, finally something that makes sense!

    // I mean
    let x = {{{{{Some(1)}}}}};

    // and I got a nice explicit return a semi-colon offical!
    foo_the_c_way();
}

fn rule_4() {

}

fn foo_the_c_way() -> i32 {

    return 1;
}
