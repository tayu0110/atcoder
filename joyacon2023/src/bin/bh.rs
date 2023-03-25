use itertools::Itertools;
use proconio::input;

fn main() {
    input! {n: usize, q: usize, x: [usize; q]}

    let mut balls = (0..n).collect::<Vec<_>>();
    let mut pos = (0..n).collect::<Vec<_>>();

    for x in x {
        let p = pos[x - 1];

        if p == n - 1 {
            let nei = balls[p - 1];
            balls.swap(p, p - 1);
            pos.swap(x - 1, nei);
        } else {
            let nei = balls[p + 1];
            balls.swap(p, p + 1);
            pos.swap(x - 1, nei);
        }
    }

    println!("{}", balls.iter().map(|v| *v + 1).join(" "))
}
