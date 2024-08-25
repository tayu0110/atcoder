use proconio::input;

fn main() {
    input! {q: usize}

    let mut map = std::collections::BTreeMap::new();
    for _ in 0..q {
        input! {t: usize}

        if t == 1 {
            input! {x: usize}
            *map.entry(x).or_insert(0usize) += 1;
        } else if t == 2 {
            input! {x: usize, c: usize}
            let entry = map.entry(x).or_insert(0);
            *entry = entry.saturating_sub(c);
            if *entry == 0 {
                map.remove(&x);
            }
        } else if let Some((min, _)) = map.iter().next() {
            if let Some((max, _)) = map.iter().next_back() {
                println!("{}", max - min);
            }
        }
    }
}
