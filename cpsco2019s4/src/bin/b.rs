use proconio::*;

fn main() {
    input! {n: usize, d: usize, s: [marker::Chars; d]}

    let mut res = 0;
    for i in 0..1usize << d {
        if i.count_ones() != 2 {
            continue;
        }

        let mut t = vec![false; n];
        for j in 0..d {
            if i & 1 << j != 0 {
                for k in 0..n {
                    if s[j][k] == 'o' {
                        t[k] = true;
                    }
                }
            }
        }

        res = res.max(t.into_iter().filter(|&f| f).count());
    }

    println!("{}", res)
}
