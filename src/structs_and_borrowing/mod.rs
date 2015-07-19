// I need a function that takes ownership of a struct
// and returns another struct, from the first structs attributes

// So I want to create some Structs from attributes from other structs,
// and I don't want to clone

pub fn an_exploration() {
    println!("Structs, their attributes and borrowing");

    let user1 = User1 { name: "Bill".to_string() };
    let converted_user: User2 = user_converter(user1);

    println!("Converted User name: {}", converted_user.name);

    // println!("{:?}", user1.name); // won't compile user is moved
}

fn user_converter(user1: User1) -> User2 {
    let user2 = User2 { name: user1.name };
    user2
}

struct User1 {
    name: String,
}

struct User2 {
    name: String,
}
