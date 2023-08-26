use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let sum = a.iter().sum::<usize>();
    let ave = sum / n;

    if ave * n == sum {
        println!(
            "{}",
            a.into_iter()
                .filter(|&a| a < ave)
                .map(|a| ave - a)
                .sum::<usize>()
        );
        return;
    }

    let mut min = 0;
    let mut add = 0;
    for a in a {
        if a > ave {
            min += a - (ave + 1);
        } else {
            add += ave - a;
        }
    }

    println!("{}", min.max(add))
}
