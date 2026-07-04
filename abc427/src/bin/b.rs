use proconio::*;

fn main() {
    input! {n: usize}

    let f = |a: usize| {
        a.to_string()
            .bytes()
            .map(|b| (b - b'0') as usize)
            .sum::<usize>()
    };
    let mut a = vec![0; n + 1];
    a[0] = 1;
    for i in 1..=n {
        for j in 0..i {
            a[i] += f(a[j]);
        }
    }

    println!("{}", a[n])
}
