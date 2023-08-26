use std::collections::HashMap;

use proconio::*;

fn main() {
    input! {s: marker::Chars, t: marker::Chars}

    let mut map = HashMap::new();
    let mut k = s.iter().filter(|&&c| c == '@').count();
    for c in s {
        if c == '@' {
            continue;
        }
        *map.entry(c).or_insert(0) += 1;
    }

    for c in t {
        if c == '@' {
            continue;
        }
        if let Some(v) = map.get_mut(&c) {
            *v -= 1;

            if *v == 0 {
                map.remove(&c);
            }
            continue;
        }

        if "atcoder".contains(c) && k > 0 {
            k -= 1;
            continue;
        }

        println!("No");
        return;
    }

    for (c, _) in map {
        if !"atcoder".contains(c) {
            println!("No");
            return;
        }
    }

    println!("Yes")
}
