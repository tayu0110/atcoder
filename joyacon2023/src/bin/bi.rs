use proconio::{input, marker::Chars};

fn main() {
    input! {n: usize, k: usize, s: [Chars; n]}

    let mut res = 0;
    for i in 0u32..(1 << n) {
        if i.count_ones() < k as u32 {
            continue;
        }

        let mut c = vec![0usize; 26];
        for j in 0..n {
            if i & (1 << j) != 0 {
                for &nc in &s[j] {
                    c[(nc as u8 - b'a') as usize] += 1;
                }
            }
        }

        res = std::cmp::max(res, c.into_iter().filter(|c| *c == k).count());
    }

    println!("{}", res);
}
