use proconio::*;

fn main() {
    input! {a: usize, b: usize, c: usize, d: usize, mut e: [[usize; a]; c]}

    let mut t = vec![];
    for v in e.iter_mut() {
        v.sort();
        v.reverse();

        t.push(v[b - 1]);
    }

    t.sort();
    t.reverse();

    println!("{}", t[d - 1])
}
