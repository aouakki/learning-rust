// Variables are immutable by default
// Variables are typed, once created with a type, that type cannot be changed
// Rust is a block-scoped language

pub fn run() {
    // immutable var
    let name = "Abderrahim";
    // mutable var
    let mut age = 20;
    println!("my name is {} and my age is {}", name, age);
    age = 21;
    println!("my name is {} and my new age age is {}", name, age);

    // define constant
    const ID: i32 = 134566;
    println!("ID: {}", ID);

    // assign multiple variables
    let (hobby, when) = ("football", "weekends");
    println!("I like to play {} on {}", hobby, when);
}
