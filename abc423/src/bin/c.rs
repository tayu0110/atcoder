use proconio::*;

fn solve(mut r: usize, mut l: Vec<usize>) -> usize {
    let mut res = 0;
    while r > 0 {
        if l[r - 1] == 1 {
            l[r - 1] = 0;
            res += 1;
        }
        r -= 1;
    }
    while r < l.len() {
        if l[r] == 1 {
            l[r] = 0;
            res += 1;
        }
        r += 1;
        if l[r - 1] == 0 {
            l[r - 1] = 1;
            res += 1;
        }
    }
    res
}

fn main() {
    input! {n: usize, mut r: usize, mut l: [usize; n]}

    if l.iter().all(|&l| l == 1) {
        println!("0");
        return;
    }

    for i in 0..=r {
        if l[i] == 0 || i == r {
            l.drain(..i);
            r -= i;
            break;
        }
    }
    while l.len() > r && l.last().unwrap() == &1 {
        l.pop();
    }
    let n = l.len();

    let mut res = solve(r, l.clone());
    l.reverse();
    let r = n - r;
    res = res.min(solve(r, l));

    println!("{res}");
}
