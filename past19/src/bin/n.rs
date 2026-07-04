use proconio::*;

fn main() {
    input! {n: usize, a: [f64; n], x: f64, q: usize, query: [(usize, usize); q]}

    let mut tree = vec![(0.0, 0.0); n * 2];
    for (i, a) in a.into_iter().enumerate() {
        tree[i + n] = (a, x);
    }
    for i in (1..n).rev() {
        let (a, x) = tree[i * 2];
        let (b, y) = tree[i * 2 + 1];
        let c = a + b * x;
        let z = x * y;
        tree[i] = (c, z);
    }

    for (l, r) in query {
        let (mut l, mut r) = (n + l - 1, n + r);
        let (mut a, mut x) = (0.0, 1.0);
        let (mut b, mut y) = (0.0, 1.0);
        while l < r {
            if l & 1 != 0 {
                let (b, y) = tree[l];
                let (c, z) = (a + b * x, x * y);
                (a, x) = (c, z);
                l += 1;
            }
            if r & 1 != 0 {
                let (a, x) = tree[r - 1];
                let (c, z) = (a + b * x, x * y);
                (b, y) = (c, z);
            }
            l >>= 1;
            r >>= 1;
        }
        let res = a + b * x;
        println!("{res}");
    }
}
