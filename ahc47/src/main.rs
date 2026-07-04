use itertools::Itertools;
use proconio::*;
use rand::{thread_rng, Rng};

const N: usize = 36;
const M: usize = 12;
const L: usize = 1000_000;

fn main() {
    input! {_: usize, _: usize, _: usize, s: [(String, usize); N]}

    let mut rng = thread_rng();
    let mut a = [0; M];
    for _ in 0..M {
        let c = rng.gen_range('a'..='f');
        a.fill(0);
        for _ in 0..100 {
            a[rng.gen_range(0..M)] += 1;
        }

        println!("{c} {}", a.iter().join(" "));
    }
}
