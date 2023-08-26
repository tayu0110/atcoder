use proconio::*;

fn main() {
    input! {n: usize, a: [i32; n]}

    let mut t = vec![0u128; 200010];
    for i in 0..n {
        input! {c: usize, x: [usize; c]}
        for x in x {
            t[x] |= 1 << i;
        }
    }

    input! {q: usize}
    for _ in 0..q {
        input! {d: usize, y: [usize; d]}

        let mut k = 0;
        for y in y {
            k |= t[y];
        }

        let mut res = -1;
        let mut max = 0;
        for i in 0..n {
            if k & (1 << i) == 0 && max < a[i] {
                max = a[i];
                res = i as i32 + 1;
            }
        }

        println!("{}", res);
    }
}
