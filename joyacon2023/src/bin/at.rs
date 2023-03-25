use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut sum = 0;
    let mut res = vec![0];
    for a in a {
        sum = (sum + a) % 360;
        res.push(sum);
    }
    res.push(360);

    res.sort();
    res.dedup();

    let mut ans = 0;
    for v in res.windows(2) {
        ans = std::cmp::max(ans, v[1] - v[0]);
    }

    println!("{}", ans)
}
