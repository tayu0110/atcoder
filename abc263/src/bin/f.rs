use proconio::input;

fn main() {
    input! {n: usize, c: [[usize; n]; 1 << n]}

    let n2 = 1 << n;
    let mut dpw = vec![vec![0; 1]; n2];
    let mut dpl = vec![0; n2];

    for i in 0..n {
        let w = 1 << i;
        let nw = w << 1;

        let mut pw = vec![vec![0; nw]; n2 / nw];
        let mut pl = vec![0; n2 / nw];

        std::mem::swap(&mut dpw, &mut pw);
        std::mem::swap(&mut dpl, &mut pl);        

        for j in 0..dpl.len() {
            let l = nw * j;
            let m = l + w;
            let (lj, rj) = (j << 1, (j << 1) + 1);
            for k in 0..w {
                dpw[j][k] = std::cmp::max(dpw[j][k], pw[lj][k] + pl[rj]);
                dpl[j] = std::cmp::max(dpl[j], pw[lj][k] + pl[rj] + c[l+k][i]);

                dpw[j][w+k] = std::cmp::max(dpw[j][w+k], pw[rj][k] + pl[lj]);
                dpl[j] = std::cmp::max(dpl[j], pw[rj][k] + pl[lj] + c[m+k][i]);
            }
        }
    }

    println!("{}", dpl[0]);
}