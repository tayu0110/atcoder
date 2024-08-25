use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n], q: usize}

    let mut tree = vec![vec![]; n * 2];
    for (i, &a) in a.iter().enumerate() {
        tree[i + n].push((0usize, a));
    }
    for i in (1..n).rev() {
        let (a, b) = (tree[i * 2][0], tree[i * 2 + 1][0]);
        tree[i].push(a);
        tree[i].push(b);
        tree[i].sort_unstable();
    }
    // let mut lazy_add = vec![vec![]; n * 2];
    // let mut lazy_remove = vec![vec![]; n * 2];

    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {l: usize, r: usize, x: usize}
            let (l, r) = (l + n - 1, r + n);
            while l < r {
                if l & 1 != 0 {}
            }
        }
    }
}
