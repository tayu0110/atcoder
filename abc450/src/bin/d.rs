use proconio::*;

fn main() {
    input! {n: usize, k: usize, mut a: [usize; n]}

    let &max = a.iter().max().unwrap();
    for i in 0..n {
        let diff = max - a[i];
        a[i] += k * (diff / k);
    }

    a.sort_unstable();
    let mut ret = a[n - 1] - a[0];
    for i in 0..n {
        a[i] += k;
        ret = ret.min(a[i].abs_diff(a[(i + 1) % n]));
    }

    println!("{ret}")
}
