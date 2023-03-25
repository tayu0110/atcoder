use proconio::*;

fn main() {
    input! {s: marker::Chars}

    let l = s
        .iter()
        .enumerate()
        .filter(|&(i, &c)| i % 2 == c as usize - b'0' as usize)
        .count();
    let r = s
        .iter()
        .enumerate()
        .filter(|&(i, &c)| (i + 1) % 2 == c as usize - b'0' as usize)
        .count();

    println!("{}", std::cmp::min(l, r))
}
