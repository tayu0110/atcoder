use proconio::*;

fn main() {
    input! {_: usize, s: marker::Bytes}

    let mut rle = vec![];
    for s in s {
        match rle.last_mut() {
            Some((c, cnt)) if *c == s => *cnt += 1,
            _ => rle.push((s, 1)),
        }
    }

    let mut t = [0; 26];
    for (c, r) in rle {
        let index = c as usize - b'a' as usize;
        t[index] = t[index].max(r);
    }
    println!("{}", t.into_iter().sum::<usize>())
}
