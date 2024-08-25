use proconio::*;

fn main() {
    input! {n: i32, k: i32}

    const INV: i32 = {
        let (mut res, mut val) = (1, 8);
        let mut exp = 9;
        while exp > 0 {
            if exp & 1 != 0 {
                res *= val;
                res %= 11;
            }
            val *= val;
            val %= 11;
            exp >>= 1;
        }
        assert!(res * 8 % 11 == 1);
        res
    };

    let mut res = 0;
    for j in 1..=5 {
        let p = (k - 9 - 9 * j).rem_euclid(11) * INV % 11;
        let n = (n - 3 - p).rem_euclid(11);
        eprintln!("j: {j}, p: {p}, n: {n}");
        if n > n - 3 || p == 0 {
            continue;
        }

        res += (n + 2) / 3;
    }

    println!("{res}")
}

// 7i+j-7 | 7i+j-6 | 7i+j-5
// 7i+j   | 7i+j+1 | 7i+j+2
// 7i+j+7 | 7i+j+8 | 7i+j+9
//
// 21i+3j + 21i+3j+3 + 21i+3j+6
// = 63i + 9j + 9
// = 8i + 9j + 9 (mod 11)
