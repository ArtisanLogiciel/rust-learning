fn main() {
    let number: u32 = 98;
    let mut number_2 = number;
    number_2 = 100;
    println!("{} - {}", number, number_2);

    let s1 = "hello".to_owned();
    let mut s2 = s1.clone();
    s2.push_str(" world");
    println!("s1 is {}, s2 is {}", s1, s2);
    // scope :
    {
        let s3 = s1;
        println!("s3 is {}", s3);
    }
}
