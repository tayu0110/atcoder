use itertools::Itertools;
use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n-1], b: [usize; n-2]};

    let mut res = vec![0; n + 1];
    let mut min = vec![std::usize::MAX; n + 1];
    min[1] = 0;
    for i in 0..n - 1 {
        if i < n - 1 && min[i + 1] + a[i] < min[i + 2] {
            min[i + 2] = min[i + 1] + a[i];
            res[i + 2] = i + 1;
        }
        if i < n - 2 && min[i + 1] + b[i] < min[i + 3] {
            min[i + 3] = min[i + 1] + b[i];
            res[i + 3] = i + 1;
        }
    }

    let mut now = n;
    let mut perm = vec![];
    while now > 0 {
        perm.push(now);
        now = res[now];
    }

    perm.reverse();
    println!("{}", perm.len());
    println!("{}", perm.iter().join(" "))
}
