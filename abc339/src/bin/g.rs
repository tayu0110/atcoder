use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n], q: usize}

    let mut t = vec![vec![]; n * 2];
    for i in 0..n {
        t[i + n].push((a[i], a[i]));
    }

    for i in (1..n).rev() {
        let mut s = vec![(0, 0)];
        s.extend(&t[i << 1]);
        s.extend(&t[(i << 1) | 1]);
        s.sort_unstable();
        t[i] = s;
    }

    for i in 1..2 * n {
        let len = t[i].len();
        for j in 0..len - 1 {
            t[i][j + 1].1 += t[i][j].1;
        }
    }

    let mut prev = 0;
    for _ in 0..q {
        let (l, r, x) = {
            input! {s: usize, t: usize, u: usize}
            (s ^ prev, t ^ prev, u ^ prev)
        };

        let (mut l, mut r) = (l + n - 1, r + n);
        let mut res = 0;
        while l < r {
            if l & 1 != 0 {
                let pos = t[l].partition_point(|t| t.0 <= x);
                if pos > 0 {
                    res += t[l][pos - 1].1;
                }
                l += 1;
            }
            if r & 1 != 0 {
                let pos = t[r - 1].partition_point(|t| t.0 <= x);
                if pos > 0 {
                    res += t[r - 1][pos - 1].1;
                }
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }

        println!("{res}");
        prev = res;
    }
}
