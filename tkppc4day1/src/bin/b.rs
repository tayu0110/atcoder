use proconio::*;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    let mut t = (0..n).collect::<Vec<_>>();
    t.sort_by_key(|i| a[*i]);

    for i in (0..n).rev() {
        if a[t[i]] < k {
            println!("{}", t[i] + 1);
            return;
        }
    }

    println!("-1")
}
