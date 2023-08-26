use proconio::*;

fn main() {
    input! {n: usize, s: [usize; n], m: usize, p: [(usize, usize, usize); m]}

    let sum: usize = s.iter().sum();
    let mut t = vec![vec![]; n + 1];
    for (a, b, c) in p {
        if a == 0 {
            t[b].push(c);
        } else {
            if t[b].contains(&c) {
                println!("{} {}", s[c - 1], s[c - 1]);
                continue;
            }

            let k = n - 1 - t[b].len();
            let rem = sum - t[b].iter().map(|v| s[*v - 1]).sum::<usize>() - s[b - 1];
            if k == 1 {
                println!("{} {}", rem, rem);
                continue;
            }
            let (mut min, mut max) = (std::usize::MAX, 0);
            for i in 0..=100 {
                if rem < i {
                    break;
                }
                let rem = rem - i;
                if rem <= 100 * (k - 1) {
                    min = min.min(i);
                    max = max.max(i);
                }
            }

            println!("{} {}", min, max);
        }
    }
}
