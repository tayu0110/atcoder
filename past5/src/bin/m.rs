use proconio::*;

fn main() {
    input! {n: usize, k: i64, mut a: [i64; n]}
    for i in 0..n - 1 {
        a[i + 1] += a[i];
    }
    a.insert(0, 0);

    let (mut l, mut r) = (-1, k + 1);
    while r - l > 1 {
        let m = (r + l) / 2;

        let mut res = vec![0; n + 2];
        res[0] = 1;
        res[1] = -1;
        for i in 0..n {
            let s = a.partition_point(|&na| na - a[i] < m);
            let t = a.partition_point(|&na| na - a[i] <= k);

            if s <= n && res[i] > 0 {
                res[s] += 1;
                res[t] -= 1;
            }
            res[i + 1] += res[i];
        }

        if res[n] > 0 {
            l = m;
        } else {
            r = m;
        }
    }

    println!("{l}")
}
