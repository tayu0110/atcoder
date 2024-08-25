use proconio::*;

fn main() {
    input! {n: usize, q: usize, mut a: [i64; n]}

    a.sort_unstable();

    for _ in 0..q {
        input! {b: i64, k: usize}

        let (mut l, mut r) = (-1, 10000000000i64);
        while r - l > 1 {
            let m = (r + l) / 2;
            let p = a.partition_point(|&a| a < b);
            let mut nk = 0;
            nk += p - a[..p].partition_point(|&a| (b - a).abs() > m);
            nk += a[p..].partition_point(|&a| (a - b).abs() <= m);

            if nk < k {
                l = m;
            } else {
                r = m;
            }
        }

        println!("{r}");
    }
}
