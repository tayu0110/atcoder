use proconio::*;

fn main() {
    input! {h: usize, w: usize, k: usize, s: [marker::Chars; h]}

    let mut res = 0;
    for i in 0..1 << h {
        for j in 0..1 << w {
            let mut s = s.clone();
            for k in 0..h {
                if i & (1 << k) != 0 {
                    for p in 0..w {
                        s[k][p] = '.';
                    }
                }
            }

            for l in 0..w {
                if j & (1 << l) != 0 {
                    for p in 0..h {
                        s[p][l] = '.';
                    }
                }
            }

            if s.into_iter().flatten().filter(|&f| f == '#').count() == k {
                res += 1;
            }
        }
    }

    println!("{}", res)
}
