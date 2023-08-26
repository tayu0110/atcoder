use proconio::*;

fn main() {
    input! {n: usize, x: marker::Bytes, s: [marker::Bytes; n]}

    let mut t = vec![0; 26];
    for x in x {
        let idx = (x - b'A') as usize;
        t[idx] += 1;
        t[idx] = t[idx].min(3);
    }

    let mut s = s
        .into_iter()
        .enumerate()
        .map(|(index, s)| {
            let mut buf = vec![0; 26];
            for s in s {
                let idx = (s - b'A') as usize;
                buf[idx] += 1;
                buf[idx] = buf[idx].min(2);
            }
            buf.iter_mut().zip(&t).for_each(|(s, t)| *s += *t);
            let &max = buf.iter().max().unwrap();
            let mut res = (0, 0, 0);
            for (i, c) in buf.into_iter().enumerate() {
                if max == c {
                    res = (max, -(i as i32), -(index as i32));
                    break;
                }
            }
            res
        })
        .collect::<Vec<_>>();

    s.sort();

    let (_, _, res) = s.pop().unwrap();
    println!("{}", 1 - res)
}
