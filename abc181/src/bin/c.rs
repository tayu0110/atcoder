use proconio::*;

fn main() {
    input! {n: usize, p: [(i32, i32); n]}

    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let mut v = [p[i], p[j], p[k]];
                v.sort();

                let (a, b) = v[0];
                let (k, l) = v[1];
                let (s, t) = v[2];

                if (s - k) * (l - b) == (k - a) * (t - l) {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No")
}
