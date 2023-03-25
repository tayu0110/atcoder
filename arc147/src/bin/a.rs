#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, mut a: [usize; n]};
    a.sort();

    let mut min = a[0];
    let mut nt = a.into_iter().skip(1).collect::<std::collections::BinaryHeap<usize>>();

    let mut res = 0;
    while let Some(v) = nt.pop() {
        res += 1;
        let next = v % min;
        if next != 0 {
            if next < min {
                nt.push(min);
                min = next;
            } else {
                nt.push(next);
            }
        }
    }

    println!("{}", res);
}
