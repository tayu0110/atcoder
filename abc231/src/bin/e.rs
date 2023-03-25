use proconio::input;
use std::collections::HashMap;

fn rec(rem: usize, a: &[usize], memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(&res) = memo.get(&(rem, *a.last().unwrap())) {
        return res;
    }

    let len = a.len();
    let na = a[len - 1];
    let base = rem / na;
    let nrem = rem % na;
    if nrem == 0 {
        memo.insert((rem, na), base);
        base
    } else {
        let res1 = base + rec(nrem, &a[0..len - 1], memo);
        let res2 = base + 1 + rec(na - nrem, &a[0..len - 1], memo);
        memo.insert((rem, na), std::cmp::min(res1, res2));
        *memo.get(&(rem, na)).unwrap()
    }
}

fn main() {
    input! {n: usize, x: usize, a: [usize; n]}

    let mut memo = HashMap::new();
    println!("{}", rec(x, &a, &mut memo));
}
