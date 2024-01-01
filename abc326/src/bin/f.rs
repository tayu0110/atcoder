use proconio::*;
use rustc_hash::FxHashMap;

type HashMap<K, V> = FxHashMap<K, V>;

fn enumerate(a: &[i32]) -> HashMap<i32, u32> {
    let len = a.len();
    let mut res = HashMap::default();

    for i in 0..1 << len {
        let mut sum = 0;
        for j in 0..len {
            if i & (1 << j) == 0 {
                sum += a[j];
            } else {
                sum -= a[j];
            }
        }

        res.insert(sum, i);
    }

    res
}

fn solve(x: i32, a: &[i32]) -> Option<u64> {
    let n = a.len();

    if n == 1 {
        return (a[0] == x.abs()).then_some((a[0] != x) as u64);
    }

    let m0 = enumerate(&a[..n / 2]);
    let m1 = enumerate(&a[n / 2..]);

    for (k, &v) in &m0 {
        if let Some(&nv) = m1.get(&(x - k)) {
            return Some(v as u64 | ((nv as u64) << (n / 2)));
        }
    }

    None
}

fn main() {
    input! {n: usize, x: i32, y: i32, a: [i32; n]}

    let dy = (0..n)
        .filter(|i| i % 2 == 0)
        .map(|i| a[i])
        .collect::<Vec<_>>();
    let dx = (0..n)
        .filter(|i| i % 2 == 1)
        .map(|i| a[i])
        .collect::<Vec<_>>();

    let y = solve(y, &dy);
    let x = solve(x, &dx);

    if let (Some(y), Some(x)) = (y, x) {
        let len = dx.len();
        let mut res = String::with_capacity(n);
        let mut dx = 1;
        let mut dy;
        for i in 0..len {
            let y = y & (1 << i) == 0;
            if y ^ (dx > 0) {
                res.push('R');
            } else {
                res.push('L');
            }
            dy = if y { 1 } else { -1 };

            let x = x & (1 << i) == 0;
            if x ^ (dy > 0) {
                res.push('L');
            } else {
                res.push('R');
            }
            dx = if x { 1 } else { -1 };
        }

        if n != len << 1 {
            if (y & (1 << len) == 0) ^ (dx > 0) {
                res.push('R');
            } else {
                res.push('L');
            }
        }

        println!("Yes");
        println!("{res}");
    } else {
        println!("No");
    }
}
