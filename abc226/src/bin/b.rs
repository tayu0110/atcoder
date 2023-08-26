use proconio::*;

fn main() {
    input! {n: usize}

    let mut t = vec![];
    for _ in 0..n {
        input! {a: [usize]}
        t.push(a);
    }
    t.sort();
    t.dedup();
    println!("{}", t.len())
}
