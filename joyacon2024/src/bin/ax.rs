use itertools::Itertools;
use proconio::*;

fn main() {
    input! {mut c: [[char; 4]; 4]}

    c.reverse();
    c.iter_mut().for_each(|v| v.reverse());

    for c in c {
        println!("{}", c.iter().join(" "))
    }
}
