use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut p: [(usize, usize); m]}
    p.sort();

    let mut dist = vec![std::usize::MAX >> 10; n + 1];
    dist[1] = 0;
    for (a, b) in p {
        dist[b] = dist[b].min(dist[a] + 1);
    }

    if dist[n] > 2 {
        println!("IMPOSSIBLE")
    } else {
        println!("POSSIBLE")
    }
}
