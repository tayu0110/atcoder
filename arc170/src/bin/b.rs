use proconio::*;

fn main() {
    input! {n: usize, a: [i8; n]}

    let mut pos = vec![vec![]; 11];
    for (i, &a) in a.iter().enumerate() {
        pos[a as usize].push(i);
    }

    let mut buf = vec![];
    for i in 0..n {
        let now = a[i];
        let mut min = usize::MAX;
        for diff in -5..=5 {
            let next = now + diff;
            let next_next = next + diff;

            if next <= 0 || 10 < next || next_next <= 0 || 10 < next_next {
                continue;
            }

            let next = next as usize;
            let next_next = next_next as usize;

            let p = pos[next].partition_point(|&p| p <= i);
            if p == pos[next].len() {
                continue;
            }
            let p = pos[next][p];
            let np = pos[next_next].partition_point(|&np| np <= p);
            if np == pos[next_next].len() {
                continue;
            }
            min = min.min(pos[next_next][np]);
        }
        if min < usize::MAX {
            while let Some((front, back)) = buf.pop() {
                if min <= back {
                    continue;
                }
                buf.push((front, back));
                break;
            }
            buf.push((i, min));
        }
    }

    let mut prev = 0;
    let mut res = 0;
    for (f, b) in buf {
        res += (n - b) * (f - prev + 1);
        prev = f + 1;
    }

    println!("{}", res);
}
