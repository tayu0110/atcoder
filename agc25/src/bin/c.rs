use proconio::input;

fn solve(
    n: usize,
    f: usize,
    mut s: std::collections::BinaryHeap<(i64, i64, usize)>,
    mut t: std::collections::BinaryHeap<std::cmp::Reverse<(i64, i64, usize)>>,
) -> i64 {
    let mut res = 0;
    let mut now = 0;
    let mut used = vec![false; n];
    for i in 0..n {
        loop {
            let (l, r, i) = if i % 2 == f {
                s.pop().unwrap()
            } else {
                let std::cmp::Reverse((r, l, i)) = t.pop().unwrap();
                (l, r, i)
            };

            if !used[i] {
                used[i] = true;
                if l > now {
                    res += l - now;
                    now = l;
                } else if r < now {
                    res += now - r;
                    now = r;
                }

                break;
            }
        }
    }

    res + now.abs()
}

fn main() {
    input! {n: usize, p: [(i64, i64); n]}

    let mut high = std::collections::BinaryHeap::new();
    let mut low = std::collections::BinaryHeap::new();
    for (i, (l, r)) in p.into_iter().enumerate() {
        high.push((l, r, i));
        low.push(std::cmp::Reverse((r, l, i)));
    }

    let (l, r) = (
        solve(n, 0, high.clone(), low.clone()),
        solve(n, 1, high, low),
    );

    // eprintln!("l: {}, r: {}", l, r);

    println!("{}", std::cmp::max(l, r))
}
