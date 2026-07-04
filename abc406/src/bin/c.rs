use proconio::*;

fn main() {
    input! {n: usize, p: [usize; n]}

    let mut up = vec![];
    let mut down = vec![];
    for (i, v) in p.windows(3).enumerate() {
        if v[0] < v[1] && v[1] > v[2] {
            up.push(i);
        } else if v[0] > v[1] && v[1] < v[2] {
            down.push(i);
        }
    }
    up.push(n - 2);
    down.push(n - 2);

    let mut res = 0usize;
    for i in 0..n - 1 {
        if p[i] > p[i + 1] {
            continue;
        }

        let pup = up.partition_point(|&j| j < i);
        let pdown = down.partition_point(|&j| j < i);
        let j = up[pup].max(down[pdown]);
        if j + 3 > n {
            continue;
        }

        let k = up[pup + 1].min(down[pdown + 1]);
        if j > k {
            continue;
        }
        res += k - j;
    }

    println!("{res}")
}
