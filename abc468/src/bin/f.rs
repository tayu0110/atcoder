use proconio::*;

fn main() {
    input! {n: usize, p: [usize; n]}

    let mut history = vec![vec![]; n];
    let mut lis = vec![usize::MAX; n];
    for (i, &p) in p.iter().enumerate() {
        let pos = lis.partition_point(|&l| l < p);
        lis[pos] = lis[pos].min(p);
        history[pos].push((p, i));
    }

    let mut used = vec![];
    let mut prev = usize::MAX;
    let mut pi = usize::MAX;
    // eprintln!("history: {history:?}");
    while let Some(h) = history.pop() {
        if h.is_empty() {
            continue;
        }
        let mut max = usize::MAX;
        let mut mi = usize::MAX;
        for (p, i) in h {
            if p > prev || i > pi {
                // eprintln!("prev: {prev}, pi: {pi}, p: {p}, i: {i}");
                continue;
            }

            if p < max {
                max = p;
                mi = i;
            }
        }

        prev = max;
        pi = mi;
        used.push(mi);
    }
    used.reverse();
    eprintln!("{used:?}");
    let ret0 = used.len();

    let p = p
        .into_iter()
        .enumerate()
        .filter_map(|(i, p)| used.binary_search(&i).is_err().then_some(p))
        .collect::<Vec<_>>();
    let mut lis2 = vec![usize::MAX; n];
    for p in p {
        let pos = lis2.partition_point(|&l| l < p);
        lis2[pos] = lis2[pos].min(p);
    }

    println!(
        "{}",
        ret0 + lis2.iter().filter(|&&p| p < usize::MAX).count()
    );
}
