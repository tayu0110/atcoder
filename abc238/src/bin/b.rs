use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut now = 0;
    let mut res = vec![0, 360];
    for a in a {
        now = (now + a) % 360;
        res.push(now);
    }

    res.sort();
    println!("{}", res.windows(2).map(|v| v[1] - v[0]).max().unwrap())
}
