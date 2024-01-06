use proconio::input;

fn main() {
    input! {n: usize, k: usize, a: [i64; n]}

    let pos = a.iter().filter(|&&v| v > 0).cloned().collect::<Vec<_>>();
    let zero = a.iter().filter(|&&v| v == 0).count();
    let neg = a.into_iter().filter(|&v| v < 0).collect::<Vec<_>>();

    let zeros = zero * zero.saturating_sub(1) / 2 + zero * (n - zero) / 2;

    if pos.len() * neg.len() <= k {
    } else if pos.len() * neg.len() + zeros <= k {
        println!("0")
    } else {
        let _k = k - pos.len() * neg.len() - zeros;
    }
}
