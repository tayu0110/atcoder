use num::BigUint;
use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize, a: [String; n]}

    let mut degs = vec![0; 2010];
    for i in 0..n {
        degs[a[i].len() - 1] += 1;
    }

    let mut map = FxHashMap::default();
    for i in 0..n {
        let bi = a[i].parse::<BigUint>().unwrap();

        *map.entry(bi).or_insert(0) += 1;
    }

    let mut res = 0;
    for i in 0..n {
        let fd = a[i].chars().next().unwrap() as u8 - b'0';
        let bi = a[i].parse::<BigUint>().unwrap();
        for j in 0..n {
            let mut deg = a[i].len() + a[j].len() - 2;

            let gd = a[j].chars().next().unwrap() as u8 - b'0';
            if fd * gd >= 10 {
                deg += 1;
            }

            if degs[deg] == 0 && degs[deg + 1] == 0 {
                continue;
            }

            let mul = a[j].parse::<BigUint>().unwrap() * &bi;
            res += *map.get(&mul).unwrap_or(&0);
        }
    }

    println!("{res}")
}
