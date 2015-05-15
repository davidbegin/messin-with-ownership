extern crate type_printer;

// http://chrismorgan.info/blog/rust-ownership-the-hard-way.html

pub fn the_hard_way() {
    // rule_1();
    // rule_2();
    option_sidebar();
}

fn rule_1() {
    let rule_1: &str = "
        Each object can used exactly once.
        Once you use an object it is moved to the new location
        and is no longer usable in the old.
    ";

    println!("Rule 1: {}", rule_1);

    // So the deeper question is what constitutes "using"

    struct A;

    let a = A;
    let b = a;
    // won't compile
    // let c = a;
    let c = b;

    // This is such a small concept, but I somehow never got it,
    // and had so many problems around it, I need to go back,
    // and tackle a lot of areas I used to be struggling with.
    //
    // and thats just from reading a sentence!
    // Note to self: Tell new Rustaceans this
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

fn option_sidebar() {
    // I need to take a quick sidebar, and mess with options
    // I need a refresher

    // So what are options for?

    // initial guess: They are for values they may be nil

    // So there are 3 keywords to this Option business.
    // * Option
    // * Some
    // * None

    // So that means I can make a function that returns
    // either Some or None

    // println!("{:?}", do_not_get_greedy(5));
    // println!("{:?}", do_not_get_greedy(14));

    // we can also now make functions that take an option
    // meaning they can take either Some or None
    // you_got_to_be_flexible(None);
    // you_got_to_be_flexible(Some(10));

    let this_thang: i32 = 11;

    let num: Option<i32> = match this_thang {
        10 => Some(10),
        _ => None
    };

    // you_got_to_be_flexible(num)
    if_let_example();
}

fn you_got_to_be_flexible(num: Option<i32>) {
    if num.is_some() {
        println!("You got to be flexible!");
    } else {
        println!("I got you covered");
    }
}

fn do_not_get_greedy(num: i32) -> Option<i32> {
    if num < 10 {
      Some(num)
    } else {
      None
    }
}

fn if_let_example() {
    let option = Some(10);

    match option {
        Some(x) => { foo(x) },
        None    => {        }
    }
}

fn foo<T>(t: T) {
    println!("I can do anything I put my mind to!");
}
