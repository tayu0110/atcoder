use proconio::*;

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    let mut tm = vec![0i32; 1000010];
    for (s, t) in p {
        tm[s] += 1;
        tm[t] -= 1;
    }

    let mut cnt = 0;
    for i in 0..1000000 {
        tm[i + 1] += tm[i];

        if tm[i] == 0 && tm[i + 1] > 0 {
            cnt += 1;
        }
    }

    println!("{}", cnt)
}
