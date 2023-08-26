use proconio::*;

fn main() {
    input! {a: usize, b: usize}
    let map = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    let (mut s, mut t) = (0, 0);
    let (mut k, mut l) = (0, 0);
    for i in 0..3 {
        for j in 0..3 {
            if map[i][j] == a {
                s = i;
                t = j;
            }
            if map[i][j] == b {
                k = i;
                l = j;
            }
        }
    }

    let di = s.max(k) - s.min(k);
    let dj = t.max(l) - t.min(l);

    if di == 0 && dj == 1 {
        println!("Yes")
    } else {
        println!("No")
    }
}
