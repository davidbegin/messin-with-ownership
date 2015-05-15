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

    // SO I want to print or not print
    // match option {
    //     Some(x) => { foo(x) },
    //     None    => {        }
    // }

    // I accomplished this with match
    // mut there are other ways

    // so that is so much better
    if option.is_some() {
        let unwrapped_option = option.unwrap();
        print_something(unwrapped_option);
    }

    if let Some(x) = option {
        print_something(x);
    }

}

fn print_something<T>(generic: T) {
    println!("I can print something!");
}

fn foo<T>(t: T) {
    println!("I can do anything I put my mind to!");
}
