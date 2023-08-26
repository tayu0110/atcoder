use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n], q: usize, query: [(usize, usize); q]}

    let mut res = 0;
    let mut t = vec![0; 100010];
    for a in a {
        t[a] += 1;
        res += a;
    }

    for (b, c) in query {
        res += c * t[b];
        res -= b * t[b];

        println!("{}", res);

        t[c] += t[b];
        t[b] = 0;
    }
}
