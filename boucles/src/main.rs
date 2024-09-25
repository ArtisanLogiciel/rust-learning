use std::u8;

fn main() {
    let mut i: u8 = 0;
    println!("while");
    while i < 10 {
        if i % 2 == 0 {
            i += 1;
            continue;
        }
        println!("i = {}", i);
        i += 1;
    }
    i = 0;
    println!("loop");
    loop {
        if i % 2 == 0 {
            i += 1;
            continue;
        }
        println!("i = {}", i);
        i += 1;
        if i == 20 {
            break;
        }
    }
    i = 0;
    let result: u8 = loop {
        i += 1;
        if i == 20 {
            break i;
        }
    };
    println!("result = {}", result);
    println!("for");
    let tab: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for (index, value) in tab.iter().enumerate() {
        println!("index = {} value = {}", index, value);
    }
    println!("for with range");
    for (index, value) in (0..10).enumerate() {
        println!("index = {} value = {}", index, value);
    }
    'boucle: for i in 0..10 {
        for j in 0..10 {
            println!("i = {} j = {}", i, j);
            if i == 5 {
                break 'boucle;
            }
        }
    }
}
