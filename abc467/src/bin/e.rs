use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; n], b: [usize; n - 1]}

    let mut a = a.windows(2).map(|a| (a[0] + a[1]) % m).collect::<Vec<_>>();
    for (a, b) in a.iter_mut().zip(&b) {
        *a += m - b;
        *a %= m;
    }

    let mut diffs = vec![0];
    for i in 0..a.len() {
        let diff = (m - a[i]) % m;
        diffs.push(diff);
        if i + 1 < a.len() {
            a[i + 1] = (a[i + 1] + diff) % m;
        }
    }
    // eprintln!("diffs: {diffs:?}");

    let mut ret = diffs.iter().sum::<usize>();
    let mut t = vec![];
    for (i, &d) in diffs.iter().enumerate() {
        if i % 2 == 0 {
            t.push((d, i));
        } else {
            t.push((m - 1 - d, i));
        }
    }
    t.sort_unstable();

    let mut now = ret;
    let mut prev = m;
    while let Some((d, i)) = t.pop() {
        now += (prev - d) * (diffs.len() % 2);
        prev = d;
        if i % 2 == 0 {
            now -= m;
        } else {
            now += m;
        }
        while let Some((_, i)) = t.pop_if(|t| t.0 == d) {
            if i % 2 == 0 {
                now -= m;
            } else {
                now += m;
            }
        }

        ret = ret.min(now);
        // eprintln!("d: {d}, i: {i}, t: {t:?}, now: {now}, ret: {ret}");
    }
    println!("{ret}")
}
