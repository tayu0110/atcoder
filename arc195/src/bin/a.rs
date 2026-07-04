use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut a: [usize; n], b: [usize; m]}
    a.insert(0, usize::MAX);
    a.push(usize::MAX);

    let n = a.len();

    let mut best = vec![usize::MAX; m];
    let mut now = 0;
    for i in 0..m {
        while now < n && a[now] != b[i] {
            now += 1;
        }
        best[i] = now;
        now += 1;
    }

    if best[m - 1] >= n {
        println!("No");
        return;
    }

    best.insert(0, 0);
    best.push(n);

    for (i, v) in best.windows(3).enumerate() {
        let start = v[0] + 1;
        let end = v[2];

        if a[start..end].iter().filter(|&&a| a == b[i]).count() > 1 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
