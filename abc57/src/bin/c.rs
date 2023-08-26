use proconio::*;

fn main() {
    input! {n: usize}

    let mut res = 100;
    for i in (1..=n).take_while(|&i| i * i <= n).filter(|&v| n % v == 0) {
        let j = n / i;
        res = i.to_string().len().max(j.to_string().len());
    }

    println!("{}", res)
}
