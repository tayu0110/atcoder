use proconio::*;

const MAX: usize = 1000000;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    let mut count = vec![0; MAX + 1];
    for &a in &a {
        count[a] += 1;
    }

    let mut res = vec![1; MAX + 1];
    for i in 2..MAX + 1 {
        let mut sum = 0;
        for j in (1..MAX + 1).take_while(|j| i * j <= MAX) {
            sum += count[i * j];
        }

        if sum >= k {
            for j in (1..MAX + 1).take_while(|j| i * j <= MAX) {
                res[i * j] = res[i * j].max(i);
            }
        }
    }

    for a in a {
        println!("{}", res[a])
    }
}
