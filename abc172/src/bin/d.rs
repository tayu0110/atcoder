#[allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize}

    let mut d = vec![std::usize::MAX; n+1];
    for i in 2..=n {
        if d[i] == std::usize::MAX {
            for j in (1..=n).take_while(|j| i * *j <= n) {
                d[i*j] = i;
            }
        }
    }

    let mut res = 1usize;
    let mut map = std::collections::HashMap::new();
    for i in 2..=n {
        map.clear();

        let mut now = i;
        while now > 1 {
            *map.entry(d[now]).or_insert(0) += 1;
            now /= d[now];
        }

        let mut k = 1;
        for v in map.values() {
            k *= *v + 1;
        }

        res += i * k;
    }

    println!("{}", res);
}
