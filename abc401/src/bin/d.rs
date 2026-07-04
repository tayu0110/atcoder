use proconio::*;

fn main() {
    input! {n: usize, mut k: usize, mut s: marker::Bytes}

    for i in 0..n - 1 {
        if s[i] == b'o' && s[i + 1] == b'?' {
            s[i + 1] = b'.';
        } else if s[i] == b'?' && s[i + 1] == b'o' {
            s[i] = b'.';
        }
    }

    k -= s.iter().filter(|&&c| c == b'o').count();

    if k == 0 {
        println!(
            "{}",
            s.into_iter()
                .map(|c| if c == b'?' { '.' } else { c as char })
                .collect::<String>()
        );
        return;
    }

    let mut rle = vec![];
    for b in s {
        match rle.last_mut() {
            Some((p, cnt)) if *p == b => *cnt += 1,
            _ => rle.push((b, 1)),
        }
    }

    let mut max = 0;
    for &(c, cnt) in &rle {
        if c == b'?' {
            max += (cnt + 1) / 2;
        }
    }

    let mut res = String::new();
    if max == k {
        for (c, cnt) in rle {
            if c == b'?' {
                if cnt % 2 == 1 {
                    for i in 0..cnt {
                        if i % 2 == 0 {
                            res.push('o');
                        } else {
                            res.push('.');
                        }
                    }
                } else {
                    for _ in 0..cnt {
                        res.push('?');
                    }
                }
            } else {
                for _ in 0..cnt {
                    res.push(c as char);
                }
            }
        }
    } else {
        for (c, cnt) in rle {
            for _ in 0..cnt {
                res.push(c as char);
            }
        }
    }
    println!("{res}")
}
