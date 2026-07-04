use proconio::*;

fn main() {
    input! {a: [usize; 2], b: [usize; 2], t: usize}

    let mut p = vec![false; t];
    let mut now = 0;
    while now < t {
        p[now..t.min(now + a[0])].fill(true);
        now += a[0] + a[1];
    }

    let mut q = vec![false; t];
    let mut now = 0;
    while now < t {
        q[now..t.min(now + b[0])].fill(true);
        now += b[0] + b[1];
    }

    println!(
        "{}",
        p.into_iter()
            .zip(q.into_iter())
            .filter(|&(p, q)| p && q)
            .count()
    )
}
