use proconio::*;

fn main() {
    input! {n: usize, x: usize, a: [usize; n - 1]}

    let &max = a.iter().max().unwrap();

    for now in 0..=max {
        let mut a = a.clone();
        a.push(now);
        let &min = a.iter().min().unwrap();
        if a.iter().sum::<usize>() - max - min >= x {
            println!("{now}");
            return;
        }
    }

    println!("-1")
}
