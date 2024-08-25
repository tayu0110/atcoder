use proconio::*;
use std::collections::HashMap;

fn main() {
    input! {n: usize, k: usize, p: [marker::Chars; n]}

    let mut map = HashMap::new();
    for s in p {
        *map.entry(s[0]).or_insert(0) += 1;
    }
    let mut map = map.into_values().collect::<Vec<_>>();

    eprintln!("map: {:?}", map);

    for i in 1.. {
        let mut k = k;
        map.sort();
        map.reverse();
        for v in map.iter_mut() {
            if *v > 0 {
                *v -= 1;
                k -= 1;
            }

            if k == 0 {
                break;
            }
        }

        if k != 0 {
            println!("{}", i - 1);
            return;
        }
    }
}
