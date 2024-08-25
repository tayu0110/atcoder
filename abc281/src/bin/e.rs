use itertools::Itertools;
use proconio::input;

fn main() {
    input! {n: usize, m: usize, k: usize, a: [usize; n]}
    let a = a
        .into_iter()
        .enumerate()
        .map(|(i, a)| (a, i))
        .collect::<Vec<_>>();

    let mut b = a[0..m].to_vec();
    b.sort();
    let mut sum = b.iter().map(|(a, _)| a).take(k).sum::<usize>();

    let mut res = vec![sum];
    let mut nt = b
        .iter()
        .take(k)
        .cloned()
        .collect::<std::collections::BinaryHeap<_>>();
    let mut min_set = std::collections::HashSet::new();
    for i in 0..k {
        let (_, j) = b[i];
        min_set.insert(j);
    }
    let mut pending = b
        .iter()
        .skip(k)
        .map(|e| std::cmp::Reverse(*e))
        .collect::<std::collections::BinaryHeap<_>>();

    for i in m..n {
        pending.push(std::cmp::Reverse(a[i]));

        if min_set.contains(&(i - m)) {
            sum -= a[i - m].0;
            min_set.remove(&(i - m));

            while let Some(std::cmp::Reverse((b, j))) = pending.pop() {
                if j <= i - m {
                    continue;
                }

                min_set.insert(j);
                nt.push((b, j));
                sum += b;
                break;
            }
        }

        'base: while let Some((max, j)) = nt.pop() {
            if j <= i - m {
                continue;
            }
            if !min_set.contains(&j) {
                continue;
            }

            while let Some(std::cmp::Reverse((min, k))) = pending.pop() {
                if k <= i - m {
                    continue;
                }

                if max <= min {
                    nt.push((max, j));
                    pending.push(std::cmp::Reverse((min, k)));
                } else {
                    sum = sum + min - max;
                    pending.push(std::cmp::Reverse((max, j)));
                    nt.push((min, k));
                    min_set.remove(&j);
                    min_set.insert(k);
                }

                break 'base;
            }

            break;
        }

        res.push(sum);
    }

    println!("{}", res.iter().join(" "));
}
