use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [usize; 1 << n]}

    let mut rem = (0..1 << n).collect::<Vec<_>>();
    let mut cnt = 1;
    let mut res = vec![0; 1 << n];
    while rem.len() > 1 {
        let mut next = vec![];
        for i in (0..rem.len()).step_by(2) {
            if a[rem[i]] > a[rem[i + 1]] {
                next.push(rem[i]);
                res[rem[i + 1]] = cnt;
            } else {
                next.push(rem[i + 1]);
                res[rem[i]] = cnt;
            }
        }
        cnt += 1;
        rem = next;
    }

    res[rem[0]] = cnt - 1;
    println!("{}", res.iter().join("\n"))
}
