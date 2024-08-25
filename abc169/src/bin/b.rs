use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}
    if a.contains(&0) {
        println!("0");
        return;
    }

    let mut mul = 1usize;
    for a in a {
        let (t, f) = mul.overflowing_mul(a);
        if f {
            println!("-1");
            return;
        }

        mul = t;
    }

    if mul > 1_000_000_000_000_000_000 {
        println!("-1");
        return;
    }

    println!("{}", mul)
}
