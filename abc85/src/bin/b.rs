#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, d: [usize; n]}

    let set = d.into_iter().collect::<std::collections::HashSet<_>>();

    println!("{}", set.len())
}
