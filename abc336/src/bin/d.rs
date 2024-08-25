use proconio::*;

fn solve(n: usize, a: &[usize]) -> Vec<usize> {
    let mut f = vec![0; n];
    let mut prev = 0;
    for (i, &a) in a.iter().enumerate() {
        if prev < a {
            prev += 1;
            f[i] = prev;
        } else {
            f[i] = a;
            prev = a;
        }
    }

    f
}

fn main() {
    input! {n: usize, mut a: [usize; n]}

    let f = solve(n, &a);
    a.reverse();
    let mut b = solve(n, &a);
    b.reverse();

    let mut res = 0;
    for (f, b) in f.into_iter().zip(b) {
        res = res.max(f.min(b));
    }

    println!("{}", res);
}
