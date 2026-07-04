use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; n], b: [usize; m]}

    let mut index = vec![usize::MAX; 200010];
    for (i, &a) in a.iter().enumerate() {
        index[a] = index[a].min(i + 1);
    }

    for i in 0..200000 {
        index[i + 1] = index[i + 1].min(index[i]);
    }

    for b in b {
        println!("{}", index[b] as i64)
    }
}
