fn main() {
    let age: u8 = 18;
    if age >= 18 {
        println!("You are an adult");
    } else if age > 0 {
        println!("You are not born yet");
    } else {
        println!("You are a minor");
    }
    let result = if age >= 18 {
        "You are an adult"
    } else if age > 0 {
        "You are not born yet"
    } else {
        "You are a minor"
    };
    println!("{}", result);
    match age {
        1..17 => println!("You are an minor"),
        0 => println!("You are not born yet"),
        _ if age < 1 => println!("You are born"),
        n if n < 0 => println!("You are a not born yet"),
        19 | 21 | 35 => println!("You are a major"),
        _ => println!("You are a major"),
    }
}
