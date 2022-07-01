use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n]};

    let s = a.iter().fold(0, |sum, x| sum + *x);
    if s % 10 != 0 {
        println!("No");
        std::process::exit(0);
    }

    let mut b = vec![];
    for _ in 0..2 {
        for v in &a {
            b.push(*v);
        }
    }

    let mut l = 0;
    let mut r = 0;
    let mut sum = 0;
    while l < a.len() {
        while sum < s / 10 {
            sum += b[r];
            r += 1;
        }
        if sum == s / 10 {
            println!("Yes");
            std::process::exit(0);
        }

        sum -= b[l];
        l += 1;
    }

    println!("No");
}