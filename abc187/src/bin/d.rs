use proconio::*;

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    let mut ao = p.iter().map(|&(a, _)| a).sum::<usize>();
    let mut p = p
        .into_iter()
        .map(|(a, b)| (2 * a + b, a, b))
        .collect::<Vec<_>>();
    p.sort();
    let mut res = 0;
    let mut now = 0;
    while let Some((_, a, b)) = p.pop() {
        now += a + b;
        ao -= a;
        res += 1;

        if now > ao {
            println!("{}", res);
            return;
        }
    }
}
