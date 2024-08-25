use proconio::*;

fn main() {
    input! {n: usize, mut x: [(usize, usize); n]}

    x.push((0, 0));
    x.sort_unstable();
    let (mut prev, mut now) = x.pop().unwrap();
    now = now.max(prev);
    while let Some((a, t)) = x.pop() {
        let d = prev - a;
        now += d;
        now = now.max(t);
        prev = a;
    }

    println!("{now}")
}
