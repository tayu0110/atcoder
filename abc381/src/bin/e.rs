use proconio::*;

#[fastout]
fn main() {
    input! {n: usize, q: usize, s: marker::Bytes, query: [(u32, u32); q]}

    let mut one = vec![0; n + 1];
    let mut two = vec![0; n + 1];
    let mut slash = vec![];
    for (i, &c) in s.iter().enumerate() {
        one[i + 1] = one[i] + (c == b'1') as i32;
        two[i + 1] = two[i] + (c == b'2') as i32;
        if c == b'/' {
            slash.push(i);
        }
    }

    for (l, r) in query.into_iter().map(|(l, r)| (l as usize - 1, r as usize)) {
        let pos = slash.partition_point(|&i| i < l);
        let mut res = 0;
        for &p in slash.iter().skip(pos).take_while(|&&i| i < r) {
            let half = (one[p] - one[l]).min(two[r] - two[p]);
            res = res.max(1 + half * 2);
        }

        println!("{res}")
    }
}
