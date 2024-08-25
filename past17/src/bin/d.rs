use proconio::*;

fn main() {
    input! {a: usize, b: usize, c: usize, x: [usize; 12]}

    let mut res = x.iter().map(|&x| x.saturating_sub(3) * a).sum::<usize>();
    let mut sum = 0;
    for i in 0..12 {
        res = res.min(
            sum + c.min(
                x[i..]
                    .iter()
                    .map(|&x| x.saturating_sub(50) * a)
                    .sum::<usize>()
                    + b,
            ),
        );
        sum += x[i].saturating_sub(3) * a;
    }

    println!("{res}")
}
