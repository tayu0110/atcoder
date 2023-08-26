use proconio::*;

fn main() {
    input! {n: usize, s: marker::Chars}

    let mut rle = vec![];
    for c in s {
        match rle.last_mut() {
            Some((pc, cnt)) if *pc == c => *cnt += 1,
            _ => rle.push((c, 1)),
        }
    }

    let mut res = 0usize;
    let mut t = vec![];
    for (c, cnt) in rle {
        if c == '-' {
            res += cnt;
        } else {
            t.push(cnt);
        }
    }

    if t.is_empty() {
        println!("{}", n as f64 - 0.5);
        return;
    }

    t.sort();
    let max = t.pop().unwrap();

    let mut v = vec![0.0; max + 2];
    for i in 0..max + 1 {
        v[i + 1] = v[i] + 1.0 / (i + 2) as f64;
    }

    println!(
        "{}",
        res as f64 - 1.0 + v[max + 1] + t.into_iter().map(|t| v[t]).sum::<f64>()
    )
}
