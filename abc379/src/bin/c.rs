use proconio::*;

fn main() {
    input! {n: usize, m: usize, x: [usize; m], a: [usize; m]}

    if a.iter().sum::<usize>() != n {
        println!("-1");
        return;
    }

    let mut p = x.into_iter().zip(a).collect::<Vec<_>>();
    p.sort_unstable_by_key(|&(x, _)| x);

    let mut res = 0usize;
    let mut next = 1;
    for (x, a) in p {
        if next < x {
            println!("-1");
            return;
        }
        if x < next {
            res += a * (next - x);
        }
        next += a;
        res += a * (a - 1) / 2;
    }

    println!("{res}")
}
