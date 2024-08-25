use proconio::*;
use std::collections::HashMap;

fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.sort();
    let mut map = HashMap::new();
    for &a in &a {
        *map.entry(a).or_insert(0) += 1;
    }

    let mut f = vec![false; 1000001];
    let mut res = 0;
    for a in a {
        if *map.get(&a).unwrap() == 1
            && !(1..=a)
                .take_while(|&i| i * i <= a).any(|i| a % i == 0 && (f[i] || f[a / i]))
        {
            res += 1;
        }
        f[a] = true;
    }

    println!("{}", res)
}
