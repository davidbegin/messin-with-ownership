// I find myself struggling with
// how, when, why to use borrowed or other
// ownership patterns when using iterators

extern crate type_printer;

pub fn land_of_confusion() {
    println!("this is the land of confusion!");

    // first lets make a vec
    let numbas: Vec<i32> = vec![1, 2, 3, 4];

    // then iterator over it and increase all the numbers

    let bigger_numbas = numbas.iter().map(|i| i + 1);

    // Now I can't print this
    // println!("bigger numbas: {:?}", bigger_numbas);
    println!("bigger numbas: {:?}", bigger_numbas.collect::<Vec<i32>>());

    // so what happened here?

    let bigger_numbas = numbas.iter().map(
      |i|
      {
        type_printer::print_type_of(&i);
        i + 1
      }
    );

    bigger_numbas.collect::<Vec<i32>>();

    vec![1, 2, 3].iter().map(|i| i + 1);
    // i is &'static i32

    vec![1, 2, 3].iter().map(|&i| i + 1);
    // i is i32

    // so if we don't borrow the value,
    // each value is giving a static lifetime value

    // if we borrow, then it has a normal lifetime
}
