use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n]}

    let v = a.into_iter().fold([0; 9], |mut s, v| {
        s[std::cmp::min(8, v / 400)] += 1;
        s
    });

    let mut min = v.iter().take(8).filter(|&&v| v > 0).count();
    let max = min + v[8];
    if min == 0 && v[8] > 0 {
        min = 1;
    }

    println!("{} {}", min, max)
}
