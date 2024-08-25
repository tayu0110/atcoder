use proconio::*;

fn main() {
    input! {n: usize, t: usize, e: usize, x: [usize; n]}

    for (i, x) in x.into_iter().enumerate() {
        let f = t / x * x;
        let c = f + x;
        if f.abs_diff(t) <= e || c.abs_diff(t) <= e {
            println!("{}", i + 1);
            return;
        }
    }

    println!("-1")
}
