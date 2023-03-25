#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: [usize; 5]};

    let mut ck = std::collections::HashSet::new();
    for v in a {
        ck.insert(v);
    }

    println!("{}", ck.len())
}
