use proconio::*;

fn main() {
    input! {n: usize, p: [(usize, usize, usize, usize); n]}

    let mut map = vec![vec![0; 120]; 120];
    for (a, b, c, d) in p {
        for x in a..b {
            for y in c..d {
                map[y][x] = 1;
            }
        }
    }

    println!("{}", map.iter().flatten().sum::<usize>())
}
