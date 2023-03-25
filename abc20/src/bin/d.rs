#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: i64, k: i64};

    let mut divisors = vec![];
    for i in 1..=k {
        if k % i == 0 {
            divisors.push(i);
            if k != i * i {
                divisors.push(k / i);
            }
        }

        if i * i > k {
            break;
        }
    }
    divisors.sort();
    divisors.reverse();

    let mut res = vec![];

    for (i, v) in divisors.iter().enumerate() {
        let d = n / *v;
        let mut tres = d * (d + 1) / 2 * *v;
        for j in 0..i {
            if divisors[j] % *v == 0 {
                tres -= res[j] * divisors[j];
            }
        }
        res.push(tres / *v);
    }

    println!("{}", (res.into_iter().fold(0i64, |s, v| (s + v) % 1000000007) * k) % 1000000007);
}
