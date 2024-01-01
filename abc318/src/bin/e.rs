use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut left = vec![0; n + 1];
    let mut right = vec![0; n + 1];
    for &a in &a {
        right[a] += 1;
    }

    let mut res = 0usize;
    let mut now = 0usize;
    for a in a {
        res += now - left[a] * right[a];
        now -= left[a] * right[a];
        left[a] += 1;
        right[a] -= 1;
        now += left[a] * right[a];
    }

    println!("{res}")
}
