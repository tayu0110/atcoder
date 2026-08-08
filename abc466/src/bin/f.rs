use itertools::Itertools;
use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {t: usize}

    let mut buf = vec![];
    for _ in 0..t {
        input! {n: usize, x: usize, a: [usize; n]}

        let mut b = vec![x];
        for a in a {
            if a < *b.last().unwrap() {
                b.push(a);
            } else if b.len() == 1 && b[0] == a {
                b.push(a);
            }
        }

        if b.len() == 1 {
            buf.push(0);
            continue;
        }
        b.remove(0);

        let mut segments = vec![FxHashMap::default(); b.len() + 1];
        segments[0].insert(x, 1usize);
        for (i, &nb) in b.iter().enumerate() {
            let (pre, post) = segments.split_at_mut(i + 1);
            for (seg, cnt) in pre[i].drain() {
                let d = seg / nb;
                let m = seg % nb;
                *post[0].entry(nb).or_insert(0) += cnt * d;
                if m != 0 {
                    let pos = b.partition_point(|&b| b > m);
                    *post[pos - i - 1].entry(m).or_insert(0) += cnt;
                }
            }
        }
        // eprintln!("segments: {segments:?}");
        let mut ret = segments[b.len()].iter().map(|s| s.1).sum::<usize>();
        if b.into_iter().fold(x, |x, b| x % b) != 0 {
            ret -= 1;
        }
        buf.push(ret);
    }

    println!("{}", buf.iter().join("\n"));
}
