use iolib::*;
use rustc_hash::FxHashMap;

type HashMap<K, V> = FxHashMap<K, V>;

fn main() {
    scan!(n: usize, mut p: i32, q: i32, a: [i32; n]);

    if (p + q) % 2 != 0 {
        println!("0");
        return;
    }

    let ax = (p + q) / 2;
    p -= ax;

    let mut res = 0usize;
    let mut map = HashMap::default();
    let mut cnt = 0;
    for &a in &a {
        if cnt > 0 {
            res += *map.get(&(p - a)).unwrap_or(&0);
            *map.entry(a).or_insert(0) += cnt;
        }
        cnt += (a == ax) as usize;
    }

    putln!(res);
}
