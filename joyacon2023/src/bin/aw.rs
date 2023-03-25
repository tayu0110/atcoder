use proconio::input;

fn main() {
    input! {n: usize, mut p: [usize; n-1]}
    p.insert(0, 0);

    let mut res = 0;
    let mut now = n - 1;
    while now != 0 {
        res += 1;
        now = p[now] - 1;
    }

    println!("{}", res);
}
