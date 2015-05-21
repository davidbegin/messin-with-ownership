// I find myself struggling with
// how, when, why to use borrowed or other
// ownership patterns when using iterators

extern crate type_printer;

pub fn land_of_confusion() {
    // first lets make a vec
    let numbas: Vec<i32> = vec![1, 2, 3, 4];

    // then iterator over it and increase all the numbers

    let bigger_numbas = numbas.iter().map(|i| i + 1);

    // Now I can't print this
    // println!("bigger numbas: {:?}", bigger_numbas);
    // println!("bigger numbas: {:?}", bigger_numbas.collect::<Vec<i32>>());

    // so what happened here?

    let bigger_numbas = numbas.iter().map(|i| i + 1);
    bigger_numbas.collect::<Vec<i32>>();

    vec![1, 2, 3].iter().map(|i| i + 1);
    // i is &'static i32

    vec![1, 2, 3].iter().map(|&i| i + 1);
    // i is i32

    // so if we don't borrow the value,
    // each value is giving a static lifetime value

    // if we borrow, then it has a normal lifetime

    // so I need to learn more about lifetimes!

    // alright lets try and kick it up a notch all iterate and call some methods

    vec![1, 2, 3].iter().map(|i| {
        // won't compile
        //     expected `i32`,
        //     found `&_`
        // numba_increaser_1(i);


        // so i is borrowed value
        numba_increaser_2(i);
    });
}

fn numba_increaser_1(num: i32) -> i32 {
    5
}

fn numba_increaser_2(num: &i32) -> i32 {
    5
}
