// I find myself struggling with
// how, when, why to use borrowed or other
// ownership patterns when using iterators

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
        println!("ha!");
        i + 1
      }
    );

    bigger_numbas.collect::<Vec<i32>>();
}
