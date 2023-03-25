#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn solve(l: usize, x: usize, now: &mut usize, p: &Vec<usize>, pb: &Vec<usize>) -> usize {
    if l == 0 {
        *now += 1;
        return 1;
    }

    let mut res = 0;
    if *now + 1 <= x {
        *now += 1;
    }

    if *now + pb[l-1] <= x {
        *now += pb[l-1];
        res += p[l-1];
    } else if *now < x {
        res += solve(l-1, x, now, p, pb);
    }

    if *now + 1 <= x {
        *now += 1;
        res += 1;
    }

    if *now + pb[l-1] <= x {
        *now += pb[l-1];
        res += p[l-1];
    } else if *now < x {
        res += solve(l-1, x, now, p, pb);
    }

    if *now + 1 <= x {
        *now += 1;
    }

    res
}

fn main() {
    input! {n: usize, x: usize}

    let mut p = vec![1usize; 51];
    let mut pb = vec![1usize; 51];
    for i in 1..=50 {
        p[i] = p[i-1] * 2 + 1;
        pb[i] = pb[i-1] * 2 + 3;
    }

    println!("{}", solve(n, x, &mut 0, &p, &pb));
}
