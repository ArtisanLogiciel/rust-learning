fn main() {
    let mut string = String::from("Hello");
    {
        let string_2 = &mut string; // aucun pointeur ne peux être utilisé après un pointeur mutable
        string_2.push_str(" world !");
        println!("string_2 is {}", string_2);
    }
    let string_3 = &string;
    println!("string_3 is {}", string_3);
    println!("string is {}", string);
}
