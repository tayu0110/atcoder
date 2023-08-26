use proconio::input;

fn main() {
    input! {d: usize, n: usize, p: [(usize, usize); n]}
    let mut res = vec![0i64; d + 1];
    for (l, r) in p {
        res[r] -= 1;
        res[l - 1] += 1;
    }
    for i in 0..d {
        println!("{}", res[i]);
        res[i + 1] += res[i];
    }
}
