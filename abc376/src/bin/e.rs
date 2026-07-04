use ds::BTreeMultiSet;
use itertools::Itertools;
use proconio::*;
use rustc_hash::FxHashSet;

fn main() {
    input! {t: usize}

    let mut buf = vec![];
    for _ in 0..t {
        input! {n: usize, k: usize, a: [usize; n], b: [usize; n]}

        let mut a = a
            .into_iter()
            .enumerate()
            .map(|(i, a)| (a, i))
            .collect::<Vec<_>>();
        a.sort_unstable();

        let mut bs = b
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, b)| (b, i))
            .collect::<Vec<_>>();
        bs.sort_unstable();
        bs.reverse();
        let mut sum = 0;
        let mut t = FxHashSet::default();
        let mut s = BTreeMultiSet::new();
        while let Some((b, i)) = bs.pop() {
            sum += b;
            t.insert(i);
            s.insert(b);
            if t.len() == k {
                break;
            }
        }

        let mut trash = vec![false; n];
        let mut res = usize::MAX;
        'b: while let Some((a, i)) = a.pop() {
            trash[i] = true;

            if t.contains(&i) {
                res = res.min(sum * a);

                sum -= b[i];
                t.remove(&i);
                s.remove(&a);
                while let Some((b, j)) = bs.pop() {
                    if trash[j] {
                        continue;
                    } else {
                        t.insert(j);
                        s.insert(b);
                        sum += b;
                        continue 'b;
                    }
                }
                break;
            } else {
                res = res.min((sum + b[i] - s.last().unwrap()) * a);
            }
        }
        buf.push(res);
    }

    println!("{}", buf.iter().join("\n"))
}
