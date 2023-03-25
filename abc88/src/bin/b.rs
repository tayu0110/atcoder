#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.sort_by_key(|v| std::cmp::Reverse(*v));

    let sum = a.iter().sum::<usize>();
    let alice = a.iter().enumerate().filter(|(i, _)| *i % 2 == 0).map(|(_, v)| *v).sum::<usize>();
    let bob = sum - alice;

    println!("{}", alice - bob);
}
