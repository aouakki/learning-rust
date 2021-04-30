/**
 * Primitives types :
 * - integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, u128
 * - Floats: f32, f64
 * - Booleans: true, false
 */

/**
* rust is statically typed language, the compiler must now the type of all
* the variable at the compile time,if it's not set explicity,
* it will try to infer the type based on the assigned value
*/

pub fn run() {
    // the compiler warns about unused variable
    // to ignore this prix the variable name with underscore: "_"

    // default is i32
    let _x = 1;
    // default is f64
    let _y = 2.5;

    // explicit types
    let _z: u8 = 2;
    let _w: f32 = 1.5;

    // find max int
    println!("Max i32 : {}", std::i32::MAX);
    println!("Max f32 : {}", std::f32::MAX);

    // rust has a snale case naming convention for variables and functions
    let is_alive = true;

    println!("is he alive? {}", is_alive);

    // boolean from the expression
    let still_alive = !is_alive;
    println!("still alive? {}", still_alive);

    // chars are represented in unicode
    let chr = "❤️";
    println!("this is {}", chr);

    // types casting : rust cannot inplicitly cast variable however
    // it's possible but explicit cast can be always done

    let price: i32 = 100;
    let f_print: f32 = price as f32;
    println!("the price is {:.2}", f_print);

    // custom types : rust has 2 keywords struct and enum
    // to print the whole struct, we have to add a Debug trait (think of it as annotations in Java)
    // NB: struct and enum are CamelCase
    #[derive(Debug)]
    struct Person {
        name: String,
        age: i32,
        balance: f64,
        is_alive: bool,
    }
    let user = Person {
        name: String::from("Joe Doe"),
        age: 34,
        balance: 5999.28,
        is_alive: true,
    };
    println!("our user is {:?}", user);
    println!(
        "our user is {:?}",
        (user.name, user.age, user.balance, user.is_alive)
    );

    // enum
    #[derive(Debug)]
    enum Currencies {
        _USD,
        EUR,
    }
    #[derive(Debug)]
    struct Account {
        balance: f64,
        currency: Currencies,
    }

    let my_account = Account {
        balance: std::f64::MAX,
        currency: Currencies::EUR,
    };
    println!("my account {:?}", my_account);
}
