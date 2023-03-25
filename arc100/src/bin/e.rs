use proconio::*;

fn main() {
    input! {n: usize, a: [usize; 1 << n]}

    let mut t = vec![vec![0; 3]; 1 << n];
    for i in 0..1 << n {
        let mut now = i;
        while now > 0 {
            if t[i][0] < a[now] {
                t[i][0] = a[now];
                t[i].sort();
            }
            now = i & (now - 1);
        }
        t[i][0] = a[now];
        t[i].sort();
    }

    let mut res = vec![0; 1 << n];
    for i in 0..1 << n {
        res[i] = t[i][1] + t[i][2];
        if i > 0 {
            res[i] = res[i].max(res[i - 1]);
            println!("{}", res[i])
        }
    }
}
