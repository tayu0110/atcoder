use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut t = vec![vec![10000]; 110];
    let mut grundy = vec![0; 110];
    t[1] = vec![1];
    for i in 1..=100 {
        t[i].sort_unstable();
        t[i].dedup();
        let mut g = 0;
        for &t in &t[i] {
            if t != g {
                grundy[i] = g;
                break;
            }
            g += 1;
        }

        for j in (2..).take_while(|&j| i * j < 110) {
            t[i * j].push(grundy[i]);
        }
    }

    if a.into_iter().fold(0, |s, v| s ^ grundy[v]) == 0 {
        println!("No")
    } else {
        println!("Yes")
    }
}
