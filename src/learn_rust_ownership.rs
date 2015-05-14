pub fn the_hard_way() {
    // I like format for multi-line strings
    // prints how I want and is intutive to me
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
