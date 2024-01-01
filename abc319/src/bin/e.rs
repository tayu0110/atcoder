use proconio::*;

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y != 0 {
        x %= y;
        (x, y) = (y, x);
    }
    x
}

fn lcm(x: usize, y: usize) -> usize {
    x / gcd(x, y) * y
}

fn main() {
    input! {n: usize, x: usize, y: usize, e: [(usize, usize); n-1], q: usize, query: [usize; q]}

    let l = (1..9).fold(1, lcm);
    let mut dist = vec![vec![0; l]; n];
    for l in 0..l {
        dist[0][l] = l;
        for i in 0..n - 1 {
            let now = dist[i][l];
            let (p, t) = e[i];
            let start = (now + p - 1) / p * p;
            dist[i + 1][l] = start + t;
        }
    }

    for q in query {
        println!("{}", dist[n - 1][(q + x) % l] - ((q + x) % l) + q + x + y)
    }
}
