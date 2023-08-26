use proconio::*;

fn main() {
    input! {_: usize, k: usize, s: marker::Chars}

    let mut cnt = 0i64;
    let mut res = vec![];
    for c in s {
        res.push(cnt);
        if c == '(' {
            cnt += 1;
        } else {
            cnt -= 1;
        }
    }

    res.sort_by_key(|&v| std::cmp::Reverse(v));
    println!("{}", res[..k].iter().sum::<i64>())
}
