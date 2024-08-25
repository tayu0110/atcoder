use proconio::*;
use std::collections::HashMap;

fn main() {
    input! {n: usize, mut a: [usize; n]}

    a.sort();
    let mut map = HashMap::new();
    for &a in &a {
        *map.entry(a).or_insert(0) += 1;
    }

    for a in a {
        let mut remove = vec![];
        if map.contains_key(&a) && map.contains_key(&(a + 1)) && map.contains_key(&(a + 2)) {
            for a in [a, a + 1, a + 2] {
                *map.entry(a).or_insert(0) -= 1;
                if *map.get(&a).unwrap() == 0 {
                    remove.push(a);
                }
            }
        }

        for r in remove {
            map.remove(&r);
        }
    }

    println!("{}", map.values().sum::<usize>())
}
