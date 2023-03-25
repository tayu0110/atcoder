#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

fn solve(start: i32, x: i32, a: &[i32]) -> bool {
    let mut set = std::collections::HashSet::new();
    set.insert(start);

    for a in a {
        let mut tmp = std::collections::HashSet::new();

        for now in &set {
            tmp.insert(*now + *a);
            tmp.insert(*now - *a);
        }

        std::mem::swap(&mut tmp, &mut set);
    }

    set.contains(&x)
}

#[fastout]
fn main() {
    input! {n: usize, x: i32, y: i32, a: [i32; n]}

    let b = a.iter().enumerate().skip(1).filter(|(i, _)| i % 2 == 0).map(|(_, c)| *c).collect::<Vec<_>>();
    let c = a.iter().enumerate().filter(|(i, _)| i % 2 == 1).map(|(_, c)| *c).collect::<Vec<_>>();

    if solve(a[0], x, &b) && solve(0, y, &c) {
        println!("Yes");
    } else {
        println!("No");
    }
}
