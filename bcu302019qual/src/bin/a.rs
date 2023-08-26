use proconio::*;

fn main() {
    input! {n: usize, mut p: usize, a: [usize; n]}

    for (i, a) in a.into_iter().enumerate() {
        if p < a {
            println!("{}", i);
            return;
        }

        p -= a;
    }

    println!("{}", n);
}
