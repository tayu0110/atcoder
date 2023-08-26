use proconio::*;

fn main() {
    input! {s: marker::Chars}

    let mut rle = vec![];
    for c in s {
        match rle.last_mut() {
            Some((nc, cnt)) if *nc == c => *cnt += 1,
            _ => rle.push((c, 1)),
        }
    }

    let len = rle.len();
    let mut res = String::new();
    for i in 0..len - 1 {
        if rle[i].0 == 'W' && rle[i + 1].0 == 'A' {
            res.push('A');
            for _ in 0..rle[i].1 - 1 {
                res.push('C');
            }
            res.push('C');
            rle[i + 1].1 -= 1;
        } else {
            for _ in 0..rle[i].1 {
                res.push(rle[i].0);
            }
        }
    }

    if rle[len - 1].1 != 0 {
        for _ in 0..rle[len - 1].1 {
            res.push(rle[len - 1].0);
        }
    }

    println!("{}", res)
}
