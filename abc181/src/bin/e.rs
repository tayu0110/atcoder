use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut h: [usize; n], mut w: [usize; m]}
    h.sort();
    w.sort();

    let mut f = h
        .chunks_exact(2)
        .map(|v| vec![v[1] - v[0], 0])
        .flatten()
        .collect::<Vec<_>>();
    f.insert(0, 0);
    // eprintln!("h: {h:?}");
    h.reverse();
    let mut b = h
        .chunks_exact(2)
        .map(|v| vec![0, v[0] - v[1]])
        .flatten()
        .collect::<Vec<_>>();
    b.reverse();
    b.push(0);
    h.reverse();

    // eprintln!("f: {f:?}");
    // eprintln!("b: {b:?}");

    let mut sum: usize = b.iter().sum();
    let mut res = std::usize::MAX;

    for i in 0..n {
        let tar = h[i];
        let (mut l, mut r) = (0, m);
        while r - l > 1 {
            let m = (r + l) / 2;
            if w[m] <= tar {
                l = m;
            } else {
                r = m;
            }
        }

        let mut diff = tar.max(w[l]) - tar.min(w[l]);
        if l + 1 < m {
            diff = diff.min(w[l + 1] - tar);
        }
        if i + 1 < n && i > 0 && i % 2 == 1 {
            diff += h[i + 1] - h[i - 1];
        }

        // eprintln!("sum: {sum:?}, diff: {diff:?}");

        res = res.min(sum + diff);

        sum += f[i];
        sum -= b[i];
    }

    println!("{}", res)
}
