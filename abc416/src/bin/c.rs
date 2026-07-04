use proconio::*;

fn collect(now: usize, k: usize, buf: &mut [usize; 10], s: &[String], res: &mut Vec<String>) {
    if now == k {
        let mut b = String::new();
        for i in 0..k {
            b.push_str(&s[buf[i]]);
        }
        res.push(b);
        return;
    }

    let n = s.len();
    for i in 0..n {
        buf[now] = i;
        collect(now + 1, k, buf, s, res);
    }
}

fn main() {
    input! {n: usize, k: usize, x: usize, s: [String; n]}

    let mut res = vec![];
    collect(0, k, &mut [0; 10], &s, &mut res);
    res.sort_unstable();

    println!("{}", res[x - 1])
}
