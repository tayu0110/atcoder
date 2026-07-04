use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [usize; m], pay: [(usize, usize, usize); n - 1]}

    let mut cum = vec![0i32; n + 1];
    for p in p.windows(2) {
        let (min, max) = (p[0].min(p[1]), p[0].max(p[1]));
        cum[min] += 1;
        cum[max] -= 1;
    }
    for i in 0..n {
        cum[i + 1] += cum[i];
    }

    println!(
        "{}",
        cum.into_iter()
            .skip(1)
            .take(n - 1)
            .enumerate()
            .map(|(i, cum)| {
                let (a, b, c) = pay[i];
                (a * cum as usize).min(c + b * cum as usize)
            })
            .sum::<usize>()
    )
}
