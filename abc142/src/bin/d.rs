use math::divisors_enumeration;
use proconio::*;

fn main() {
    input! {a: u64, b: u64}

    let mut da = divisors_enumeration(a);
    da.extend(divisors_enumeration(b));
    da.sort();

    let mut d = vec![];
    for v in da.windows(2) {
        if v[0] == v[1] {
            d.push(v[0]);
        }
    }

    println!(
        "{}",
        d.into_iter()
            .filter(|&d| (2..=d).take_while(|&i| i * i <= d).all(|i| d % i > 0))
            .count()
    )
}
