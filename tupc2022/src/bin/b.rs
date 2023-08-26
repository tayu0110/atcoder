use proconio::*;

fn main() {
    input! {n: usize, m: usize, k: usize, mut p: [(usize, usize); m]}
    p.insert(0, (0, 0));
    p.push((n + 1, 0));

    let mut now = 0usize;
    let mut res = 0;
    for v in p.windows(2) {
        let (pa, _) = v[0];
        let (na, nb) = v[1];
        res += now.saturating_sub(k).min(na - pa);

        now = now.saturating_sub(na - pa);
        now += nb;
    }

    println!("{}", res)
}
