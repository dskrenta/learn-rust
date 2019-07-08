#![allow(dead_code)]

enum Stuff {
    Something,
    SomethingElse,
    Help(char),
    Stuff(f64),
    Int(i64),
    Person {
        code: char,
        help: bool,
        haha: String
    }
}

fn inspect(val: Stuff) {
    match val {
        Stuff::Something => println!("Something"),
        Stuff::SomethingElse => println!("Something Else"),
        Stuff::Help(help) => println!("The char: {}", help),
        Stuff::Stuff(float) => println!("The val: {}", float),
        Stuff::Int(int) => println!("Int: {}", int),
        Stuff::Person { code, help, haha } => {
            println!("code: {}, help: {}, haha: {}", code, help, haha)
        }
    }
}

fn main() {
    // println!("Hello, world!");
    /*
    let x = 2 * 34;
    println!("The value is {}", x);
    */

    let something = Stuff::Stuff(37.34);
    inspect(something);
    // println!("The val is: {}", something);
}
