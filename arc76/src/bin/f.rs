use flow::Dinic;
use proconio::input;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); n]}

    // chair_r: 0 ~ m-1, chair_l: m ~ 2*m-1, src: 2*m, dest: 2*m+1
    let (src, dst) = (2 * m, 2 * m + 1);
    let mut v = vec![0i32; 2 * m + 1];
    let mut ff = Dinic::new(2 * m + 2);
    for (l, r) in p {
        if l == 0 && r == m + 1 {
            continue;
        } else {
            v[r - 1] += 1;
            v[m + l] -= 1;
        }
    }

    for i in 0..2 * m {
        if v[i] > 0 {
            ff.set_edge(src, i, v[i] as usize);
        }
    }

    for i in 0..2 * m {
        if i + 1 < 2 * m && std::cmp::min(v[i], v[i + 1]) > 0 {
            ff.set_edge(i, i + 1, std::cmp::min(v[i], v[i + 1]) as usize);
        }
        if v[i] > 0 {
            ff.set_edge(i, dst, 1);
        }
        v[i + 1] += v[i];
    }

    let res = ff.flow(src, dst);
    println!("{}", n - res);
}
