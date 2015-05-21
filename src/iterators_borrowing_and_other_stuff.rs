// I find myself struggling with
// how, when, why to use borrowed or other
// ownership patterns when using iterators

extern crate type_printer;

pub fn land_of_confusion() {
    initial_exploration_of_iterators_and_borrowing();
    iteratoring_with_methods_example_1();
    iteratoring_with_methods_example_2();
    iteratoring_with_methods_example_3();
    for_in_and_borrowing_1();
    for_in_and_borrowing_2();
}

fn for_in_and_borrowing_2() {

    // hmmm this works fine, and the type is just i32
    // but thats not borrowed?
    for thang in vec![1, 2, 3] {
        type_printer::print_type_of(&thang);
        numba_increaser_that_takes_a_value_to_own(thang);

        // Won't Compile because it requires a borrowed value
        // numba_increaser_that_takes_a_borrowed_value(thang);
        // println!("for {} in a vec", thang);
    }
}

fn for_in_and_borrowing_1() {
    for thang in &vec![1, 2, 3] {
        type_printer::print_type_of(&thang);
        // but this prints out &'static
        //
        // is my terrible tool wrong!
        //
        // but this is a borrowed value
        // just_a_guy_trying_to_figure_out_a_type_through_compile_errors(thang);

        // println! handles defrencing I just learned!
        // println!("for {} in a vec", thang);
    }
}

fn iteratoring_with_methods_example_3() {
    let temp_vec: Vec<i32> = vec![1, 2, 3];

    temp_vec.iter().map(|i| {
        // I reached for the stars and failed!
        // numba_increaser_that_takes_a_static_reference(i)
    });

    vec![1, 2, 3].iter().map(|i| {
        // another swing and a miss!
        // let num: &'static i32 = 5;
        // numba_increaser_that_takes_a_static_reference(num)
    });
}

fn iteratoring_with_methods_example_2() {
    let result = vec![1, 2, 3].iter().map(|&i| {
        numba_increaser_that_takes_a_value_to_own(i)
    }).collect::<Vec<i32>>();

    println!("result: {:?}", result);
}

fn iteratoring_with_methods_example_1() {
    vec![1, 2, 3].iter().map(|i| {
        // won't compile
        //     expected `i32`,
        //     found `&_`
        // numba_increaser_that_takes_a_value_to_own(i);


        // so i is borrowed value
        numba_increaser_that_takes_a_borrowed_value(i);
    });

    let result = vec![1, 2, 3].iter().map(|i| {
        numba_increaser_that_takes_a_borrowed_value(i)
    }).collect::<Vec<i32>>();

    println!("result: {:?}", result);
}

fn initial_exploration_of_iterators_and_borrowing() {
    // first lets make a vec
    let numbas: Vec<i32> = vec![1, 2, 3, 4];

    // then iterator over it and increase all the numbers

    let bigger_numbas = numbas.iter().map(|i| i + 1);

    // THIS WON'T COMPILE
    // println!("bigger numbas: {:?}", bigger_numbas);

    // But this will
    println!("bigger numbas: {:?}", bigger_numbas.collect::<Vec<i32>>());

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
}

fn numba_increaser_that_takes_a_value_to_own(num: i32) -> i32 {
    num + 1
}

fn numba_increaser_that_takes_a_borrowed_value(num: &i32) -> i32 {
    num + 1
}

fn numba_increaser_that_takes_a_static_reference(num: &'static i32) -> i32 {
    1
}

fn just_a_guy_trying_to_figure_out_a_type_through_compile_errors(num: &str) {
}

