#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut map = std::collections::BTreeMap::new();
    for v in a {
        *map.entry(v).or_insert(0) += 1;
    }

    let mut t = None;
    for (k, v) in map.into_iter().rev() {
        if v >= 2 {
            if let Some(w) = t {
                println!("{}", w * k);
                std::process::exit(0);
            } else if v >= 4 {
                println!("{}", k * k);
                std::process::exit(0);
            }

            t = Some(k)
        }
    }

    println!("0");
}
