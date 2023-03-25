#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: usize, n: usize}
    const MAX: usize = 10000000;

    let mut dist = vec![std::usize::MAX; MAX];

    let mut nt = std::collections::BinaryHeap::new();
    nt.push(std::cmp::Reverse((0, 1)));

    while let Some(std::cmp::Reverse((nd, now))) = nt.pop() {
        if dist[now] != std::usize::MAX {
            continue;
        }
        dist[now] = nd;

        if now * a < MAX && dist[now*a] == std::usize::MAX {
            nt.push(std::cmp::Reverse((nd+1, now*a)));
        }

        if now % 10 != 0 {
            let mut s = now.to_string().chars().collect::<Vec<_>>();
            s.rotate_right(1);
            let ns = s.iter().collect::<String>().parse::<usize>().unwrap();
    
            if ns < MAX && dist[ns] == std::usize::MAX {
                nt.push(std::cmp::Reverse((nd+1, ns)));
            }
        }
    }

    if dist[n] == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", dist[n]);
    }
}
