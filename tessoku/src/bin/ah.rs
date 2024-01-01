use proconio::*;

fn main() {
    input! {n: usize, x: usize, y: usize, a: [usize; n]}

    let mut grandy = vec![0; 100001];
    for now in x..100001 {
        let s = grandy[now - x];
        if now < y {
            grandy[now] = 1 - s;
        } else {
            let t = grandy[now - y];

            if s == 0 && t == 0 {
                grandy[now] = 1;
            } else if s > 0 && t > 0 {
                grandy[now] = 0;
            } else if (s == 0 && t == 1) || (s == 1 && t == 0) {
                grandy[now] = 2;
            } else {
                grandy[now] = 1;
            }
        }
    }

    let g = a.into_iter().map(|a| grandy[a]).fold(0, |s, v| s ^ v);
    if g == 0 {
        println!("Second")
    } else {
        println!("First")
    }
}
