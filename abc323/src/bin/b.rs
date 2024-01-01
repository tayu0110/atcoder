use proconio::*;

fn main() {
    input! {n: usize, s: [marker::Chars; n]}

    let mut res = vec![];
    for (i, c) in s.into_iter().enumerate() {
        res.push((c.into_iter().filter(|&c| c == 'o').count(), -(i as i32)))
    }

    res.sort();
    res.reverse();

    for (_, res) in res {
        println!("{}", -res + 1)
    }
}
