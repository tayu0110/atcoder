use proconio::*;

fn main() {
    input! {n: usize, k: usize, sx: i64, sy: i64, tx: i64, ty: i64}

    let mut cost = (0..n)
        .map(|_| {
            input! {p: i64, q: i64, r: i64, w: i64}
            let f = p * sx + q * sy > r;
            let g = p * tx + q * ty > r;
            w & -((f ^ g) as i64)
        })
        .collect::<Vec<_>>();
    cost.sort_unstable();
    println!("{}", cost.iter().take(k).sum::<i64>())
}
