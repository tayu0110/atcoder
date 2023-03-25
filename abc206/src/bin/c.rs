#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut res = 0;
    let mut map = std::collections::HashMap::new();
    for (i, v) in a.into_iter().enumerate() {
        let mut rem = i;
        if let Some(w) = map.get(&v) {
            rem -= w;
        }

        res += rem;
        *map.entry(v).or_insert(0) += 1;
    }

    println!("{}", res);
}
