use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn main() {
    input! {n: usize, mut a: [usize; n]}
    let max = *a.iter().min().unwrap();

    let mut map = std::collections::HashMap::new();
    for v in a {
        for w in (1..=v).take_while(|w| *w * *w <= v).filter(|w| v % *w == 0) {
            let (l, r) = (w, v / w);
            if l <= max {
                let m = *map.entry(l).or_insert(v);
                map.insert(l, gcd(v, m));
            }
            if r <= max && l != r {
                let m = *map.entry(r).or_insert(v);
                map.insert(r, gcd(v, m));
            }
        }
    }

    // eprintln!("{:?}", map);
    let res = map.iter().filter(|(k, v)| k == v).collect_vec().len();
    println!("{}", res);
}
