use proconio::*;

fn main() {
    input! {_: usize, k: usize, s: marker::Bytes}

    let mut rle = vec![];
    for c in s {
        match rle.last_mut() {
            Some((p, cnt)) if *p == c => *cnt += 1,
            _ => rle.push((c, 1)),
        }
    }

    let mut cnt = 0;
    for i in 0..rle.len() {
        if rle[i].0 == b'1' {
            cnt += 1;
        }

        if cnt == k {
            rle.swap(i - 1, i);
            break;
        }
    }

    let mut res = String::new();
    for (c, cnt) in rle {
        for _ in 0..cnt {
            res.push(c as char);
        }
    }

    println!("{res}")
}
