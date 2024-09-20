fn main() {
    let tuple = ("mod", 98, 34);
    let (operation, res) = match tuple {
        (op, x, y) if op == "mod" => ("modulo", x % y),
        (op, x, y) if op == "add" => ("addition", x + y),
        (op, x, y) if op == "mul" => ("multiplication", x * y),
        _ => ("impossible tu calculate", 0),
    };
    println!("The result of the {} is {}", operation, res);
}
