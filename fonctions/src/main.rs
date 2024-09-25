fn main() {
    let calc = add(10, 15);
    println!("calc = {}", calc);
    let mut string = String::from("Hello");
    str(&string);
    println!("str = {}", string);
    ajoute_un_point(&mut string);
    println!("str = {}", string);
    let mut number = 5;
    modif(&mut number);
    println!("number = {}", number);
}
fn add(a: i32, b: i32) -> i32 {
    a + b // return est optionnel
}

// borrowing
fn str(a: &String) {
    println!("a = {}", a);
}
fn ajoute_un_point(s: &mut String) {
    s.push_str("!");
}
fn modif(a: &mut i32) {
    *a *= 10;
}
