#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, a: [usize; n]};

    let mut itr = a.into_iter().rev();
    let (a, b) = (itr.next().unwrap(), itr.next().unwrap());

    if a - b == 1 && (a + n) % 2 == 1 {
        println!("Bob");
    } else {
        println!("Alice");
    }
}
