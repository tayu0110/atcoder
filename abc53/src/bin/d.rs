#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, mut a: [usize; n]};

    let mut map = std::collections::HashMap::new();
    for v in a {
        *map.entry(v).or_insert(0) += 1;
    }

    let rem = n - map.len();
    if rem % 2 == 1 {
        println!("{}", map.len() - 1);
    } else {
        println!("{}", map.len());
    }
}
