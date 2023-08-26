use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n], d: usize, p: [(usize, usize); d]}

    let mut left = vec![0; n + 1];
    let mut right = vec![0; n + 1];
    for (i, &a) in a.iter().enumerate() {
        left[i + 1] = a.max(left[i]);
    }
    for (i, &a) in a.iter().enumerate().rev() {
        right[i] = a.max(right[i + 1]);
    }

    for (l, r) in p {
        println!("{}", left[l - 1].max(right[r]))
    }
}
