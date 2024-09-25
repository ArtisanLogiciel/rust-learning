use std::collections::HashMap;
fn main() {
    let mut array: [u8; 4] = [1, 2, 3, 4];
    let array_2 = [[1, 2], [3, 4]];
    array[1] *= 10;
    let mut array_3 = [1, 10, 15, 20, 25];
    let slice = &mut array_3[1..5];
    slice[0] = 100;
    let mut array_4 = [1, 10, 19, 21, 100];
    let slice_2 = &mut array_4[1..4];
    slice_2[0] = 100;
    println!("array is {:?}", array);
    println!("array_2 is {:?}", array_2[0][1]);
    println!("slice is {:?}", slice);
    println!("slice_2 is {:?}", slice_2);
    array_4[0] = 100;
    println!("array_4 is {:?}", array_4);
    let tuple = (1u8, 2, 3, 4, "string");
    let tuple_2: (u8, u8, u8, f32) = (1, 2, 3, 99.9);
    println!("tuple is {:?}", tuple);
    println!("tuple_2 is {:?}", tuple_2.1);
    let tuple_3 = (1, 2, 3, (4, 90.6));
    println!("tuple_3 is {:?}", tuple_3.3);
    let tuple_4 = (1, 2, 3, (4, 90.6, (1, 2, 3)));
    println!("tuple_4 is {:?}", tuple_4.3 .2 .1);
    let tuple_5 = (1, 2, 3, (4, 90.6, (1, 2, 3, (4, 5, 6))));
    println!("tuple_5 is {:?}", tuple_5.3 .2 .3 .2);
    let tuple_6 = (1, 2, 3, (4, 90.6, ("test", "string")));
    println!("tuple_6 is {:?}", tuple_6.3 .2 .0);
    let mut vector = vec![1, 2, 3, 4];
    vector.push(5);
    vector.remove(1);
    println!("vector is {:?}", vector);
    println!("vector is {:?}", vector[2]);
    println!("vector is {:?}", vector.get(23));
    match vector.get(23) {
        Some(x) => println!("vector is {:?}", x),
        None => println!("None"),
    }
    match vector.get(3) {
        Some(x) => println!("vector is {:?}", x),
        None => println!("None"),
    }
    let verctor_2 = &vector[1..4];
    println!("vector_2 is {:?}", verctor_2);
    let mut vector_3 = Vec::new();
    vector_3.push(1);
    vector_3.push(2);
    vector_3.push(3);
    println!("vector_3 is {:?}", vector_3);
    let mut hashmap = HashMap::new();
    hashmap.insert("a", 2);
    hashmap.insert("b", 3);
    hashmap.insert("c", 4);
    println!("hashmap is {:?}", hashmap);
    println!("hashmap is {:?}", hashmap.get("c"));
    println!("hashmap is {:?}", hashmap.contains_key("c"));
    let slice = String::from("hello world");
    let slice_2 = &slice[0..4];
    println!("slice_2 is {:?}", slice_2);
}
