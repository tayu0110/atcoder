use proconio::*;
use rustc_hash::FxHashMap;

type HashMap<K, V> = FxHashMap<K, V>;

fn solve(r: u32, b: u32, w: u32, c: char, memo: &mut HashMap<(u32, u32, u32), bool>) -> bool {
    if let Some(&res) = memo.get(&(r, b, w)) {
        return res;
    }
    if c == 'R' && (r, b, w) == (1, 0, 0) {
        return true;
    }
    if c == 'B' && (r, b, w) == (0, 1, 0) {
        return true;
    }
    if c == 'W' && (r, b, w) == (0, 0, 1) {
        return true;
    }
    let mut f = false;
    if r > 1 {
        f |= solve(r - 2, b + 1, w, c, memo);
    }
    if b > 1 && !f {
        f |= solve(r + 1, b - 2, w, c, memo);
    }
    if w > 1 && !f {
        f |= solve(r, b, w - 1, c, memo);
    }
    if w > 0 && b > 0 && !f {
        f |= solve(r, b, w - 1, c, memo);
    }
    if w > 0 && r > 0 && !f {
        f |= solve(r, b, w - 1, c, memo);
    }
    if b > 0 && r > 0 && !f {
        f |= solve(r - 1, b - 1, w + 1, c, memo);
    }

    memo.insert((r, b, w), f);
    f
}

fn main() {
    input! {_: usize, c: char, a: marker::Bytes}

    let convert = |c: u8| match c {
        b'R' => 0usize,
        b'B' => 1,
        b'W' => 2,
        _ => unreachable!(),
    };
    let mut color = [0; 3];
    for s in a {
        color[convert(s)] += 1;
    }

    let [r, b, w] = color;
    let mut memo = HashMap::default();
    if solve(r, b, w, c, &mut memo) {
        println!("Yes")
    } else {
        println!("No")
    }
}
