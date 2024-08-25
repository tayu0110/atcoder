use proconio::*;
use rustc_hash::FxHashMap;

type HashMap<K, V> = FxHashMap<K, V>;

fn convert(s: String) -> String {
    let mut buf = vec![0];
    buf.extend(s.as_bytes().windows(2).map(|v| (v[1] + 26 - v[0]) % 26));
    for i in 0..s.len() - 1 {
        buf[i + 1] += buf[i];
        buf[i + 1] %= 26;
    }
    buf.into_iter()
        .map(|c| (c + b'a') as char)
        .collect::<String>()
}

fn main() {
    input! {n: usize, _: usize, s: [String; n], q: usize}

    let mut map = HashMap::default();
    for s in s {
        map.insert(convert(s.clone()), s);
    }

    for _ in 0..q {
        input! {t: String}

        println!("{}", map.get(&convert(t)).unwrap());
    }
}
