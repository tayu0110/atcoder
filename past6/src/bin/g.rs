use proconio::input;

fn main() {
    input! {n: usize, m: usize, q: usize, p: [(usize, usize, usize); m], x: [usize; q]}

    let mut t = vec![vec![]; n];
    for (a, b, c) in p {
        t[a - 1].push((b - 1, c));
        t[b - 1].push((a - 1, c));
    }
    let mut reached = vec![false; n];
    let mut nt = std::collections::BinaryHeap::new();
    let mut res = 1;
    reached[0] = true;
    for &(to, w) in &t[0] {
        nt.push(std::cmp::Reverse((w, to)));
    }
    for x in x {
        let mut buf = vec![];
        while let Some(std::cmp::Reverse((weight, to))) = nt.pop() {
            if weight <= x {
                if !reached[to] {
                    res += 1;
                    reached[to] = true;
                    for &(to, w) in &t[to] {
                        if !reached[to] {
                            buf.push((to, w));
                        }
                    }
                }
            } else {
                if !reached[to] {
                    nt.push(std::cmp::Reverse((weight, to)));
                }
                break;
            }
        }

        println!("{}", res);

        for (to, w) in buf {
            if !reached[to] {
                nt.push(std::cmp::Reverse((w, to)));
            }
        }
    }
}
