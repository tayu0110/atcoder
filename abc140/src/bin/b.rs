use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n], b: [usize; n], c: [usize; n - 1]}

    let mut res = 0;
    let mut prev = usize::MAX - 1;
    for a in a {
        res += b[a - 1];
        if prev + 1 == a {
            res += c[prev - 1];
        }
        prev = a;
    }

    println!("{res}")
}
