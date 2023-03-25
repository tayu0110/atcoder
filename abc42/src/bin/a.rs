#[allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input! {mut s: [usize; 3]}

    s.sort();

    if s == vec![5, 5, 7] {
        println!("YES");
    } else {
        println!("NO");
    }
}
