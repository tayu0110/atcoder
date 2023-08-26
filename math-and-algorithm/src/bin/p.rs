use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n]}
    println!(
        "{}",
        a.into_iter().fold(0, |mut s, mut v| {
            while v != 0 {
                s %= v;
                std::mem::swap(&mut s, &mut v);
            }
            s
        })
    )
}
