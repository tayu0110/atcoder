use proconio::*;

fn main() {
    input! {n: usize, m: usize, t: usize, a: [usize; n]}

    let mut cum = vec![0i32; t + 1];
    for a in a {
        cum[a.saturating_sub(m)] += 1;
        cum[(a + m).min(t)] -= 1;
    }

    for i in 0..t {
        cum[i + 1] += cum[i];
    }
    cum.pop();

    println!("{}", cum.iter().filter(|&&c| c == 0).count());
}
