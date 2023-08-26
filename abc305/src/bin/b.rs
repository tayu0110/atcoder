use std::collections::HashMap;

use proconio::*;

fn main() {
    input! {p: char, q: char}

    let mut map = HashMap::new();
    map.insert('A', 0i32);
    map.insert('B', 3);
    map.insert('C', 4);
    map.insert('D', 8);
    map.insert('E', 9);
    map.insert('F', 14);
    map.insert('G', 23);

    println!("{}", (*map.get(&p).unwrap() - *map.get(&q).unwrap()).abs())
}
