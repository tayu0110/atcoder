use proconio::input;

fn main() {
    input! {n: usize, m: usize, a: [usize; 2*n]}

    let mut map = std::collections::HashMap::new();
    for a in a {
        *map.entry(a).or_insert(0) ^= 1;
    }

    let mut b = 0;
    let mut used = std::collections::HashSet::new();
    for (&k, &v) in &map {
        if used.contains(&k) {
            continue;
        }

        if v == 1 {
            if m % 2 == 1 {
                println!("Alice");
                return;
            }

            let l = (k + m / 2) % m;
            let w = *map.get(&l).unwrap_or(&0);
            if w != 1 {
                println!("Alice");
                return;
            }

            used.insert(l);
            b += 1;
        }
    }

    if b % 2 == 0 {
        println!("Bob")
    } else {
        println!("Alice")
    }
}
