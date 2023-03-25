#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, mut t: usize, a: [usize; n-1], p: [(usize, usize); m]};

    let mut map = std::collections::HashMap::new();
    for (x, y) in p {
        *map.entry(x-1).or_insert(0) += y;
    }

    for i in 0..n-1 {
        if let Some(v) = map.get(&i) {
            t += *v;
        }

        if t <= a[i] {
            println!("No");
            std::process::exit(0);
        }

        t -= a[i];
    }

    println!("Yes");
}
