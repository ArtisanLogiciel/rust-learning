fn main() {
    // les différents types de variables
    // u8 u16 u23 u64 u128
    // i8 i16 i32 i64 i128
    // usize isize
    // f32 f64
    // bool
    // char
    let mut number: f32 = 18u8 as f32; // excemple de variable mutable castée
    number = 35.01;
    let hexa = 0xFFu8; // exemple de variable imutable en hexadécimal
    const PI: f64 = 3.14159265358979323846; // exemple de constante
    let chaine: &str = "Hello, world!"; // exemple de variable imutable sous forme de chaîne de caractères
    let mut chaine_2: String = String::from("Hello, world!"); // exemple de variable mutable chaîne de caractères sous forme d'objet
    chaine_2.push_str(", Rust!"); // exemple d'opération sur une chaîne de caractères
    println!(
        "The numbers is {} and the hexa is {} and PI is {}",
        number, hexa, PI
    );
    println!("{}\n{}", chaine, chaine_2);
}
