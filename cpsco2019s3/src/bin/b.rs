use proconio::*;

fn main() {
    input! {n: usize, mut m: usize, mut a: [usize; n]}

    a.sort();
    a.reverse();

    for (i, a) in a.into_iter().enumerate() {
        if a >= m {
            println!("{}", i + 1);
            return;
        }

        m -= a;
    }
}
