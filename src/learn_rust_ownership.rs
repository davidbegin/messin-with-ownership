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

    println!("{:?}", do_not_get_greedy(5));
    println!("{:?}", do_not_get_greedy(14));
}

fn do_not_get_greedy(num: i32) -> Option<i32> {
    if num < 10 {
      Some(num)
    } else {
      None
    }
}
