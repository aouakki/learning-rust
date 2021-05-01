/* two types of strings :
- str primitive, immutable , fixed size
- String = Growable, heap allocated, we can modify it.
*/

pub fn run() {
    // str type
    let myname = "abderrahim";
    // String type
    let hello = String::from("hello");

    println!("{} my name is {}", hello, myname);

    // string len
    println!("length is {}", hello.len());

    let mut hi = String::from("H");

    // appending chars
    hi.push('i');
    println!("{}", hi);

    // appending strings
    hi.push_str(" there!");
    println!("{}", hi);

    // Capacity in bytes
    println!("capacity {}", hi.capacity());

    // isEmpty
    println!("is Empty {}", hi.is_empty());

    // replace
    println!("{}", hi.replace("there!", "Mike!"));

    // iterate over a string
    for word in hi.split_whitespace() {
        println!("{}", word)
    }
    // itterate over chars
    println!("===============");
    for chr in hi.chars() {
        println!("{}", chr)
    }
    println!("===============");

    //create a string with capacity ( preallocate)
    let mut s = String::with_capacity(10);
    s.push('h');
    s.push('i');
    println!("{}", s);

    // test if equal
    println!("are they equal {}?", s.eq(&hi));

    // more methods here https://doc.rust-lang.org/std/string/struct.String.html#implementations
}
