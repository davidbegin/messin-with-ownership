// I need a function that takes ownership of a struct
// and returns another struct, from the first structs attributes

// So I want to create some Structs from attributes from other structs,
// and I don't want to clone

pub fn an_exploration() {
    println!("Structs, their attributes and borrowing");

    let user1 = User1 { name: "Bill".to_string() };
    let converted_user: User2 = user_converter(user1);

    println!("Converted User name: {}", converted_user.name);
    let user3 = User3 { name: "Hillary" };
    let result: User2  = other_user_converter(user3);

    println!("{:?}", result.name);

    // println!("{:?}", user1.name); // won't compile user is moved
}

fn user_converter(user1: User1) -> User2 {
    let user2 = User2 { name: user1.name };
    user2
}


// error: the trait `core::marker::Sized` is not implemented for the type `str`
//
// I see this error all the time and need to understand more of what is happening
//
// FROM THE DOCS:
// Types with a constant size known at compile-time.
//
// So here is a function that takes a User3
// and returns a User2
//
// it takes the name from User3 and converts to a string for User2
// and return it
//
// however core::marker::Sized need to be implemented?
fn other_user_converter(user1: User3) -> User2 {
    let user2 = User2 { name: user1.name.to_string() };
    user2
}

// So not even this will work, because this error is
// coming from the args for the function
//
// lets see what type str is
// fn other_user_converter(user1: User3) -> User2 {
//     let user2 = User2 { name: "".to_string() };
//     user2
// }

struct User3 {
    name: &'static str
}

struct User1 {
    name: String,
}

struct User2 {
    name: String,
}
