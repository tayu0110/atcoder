use proconio::input;

struct MapWrapper<K, V> { map: std::collections::BTreeMap<K, V> }
#[allow(dead_code)]
impl<K, V> MapWrapper<K, V> where K: Ord {
    fn new() -> Self { Self { map: std::collections::BTreeMap::new() } }
    fn len(&self) -> usize { self.map.len() }
    fn contains(&self, key: &K) -> bool { self.map.contains_key(key) }
    fn remove(&mut self, key: &K) { if self.map.contains_key(key) { self.map.remove(key).unwrap(); } }
    fn clear(&mut self) { self.map.clear() }
    fn is_empty(&self) -> bool { self.map.is_empty() }
    fn first(&self) -> Option<(&K, &V)> { if self.is_empty() { None } else { self.map.iter().next() } }
    fn first_mut(&mut self) -> Option<(&K, &mut V)> { if self.is_empty() { None } else { self.map.iter_mut().next() } }
    fn last(&self) -> Option<(&K, &V)> { if self.is_empty() { None } else { self.map.iter().rev().next() } }
    fn last_mut(&mut self) -> Option<(&K, &mut V)> { if self.is_empty() { None } else { self.map.iter_mut().next() } }
    fn iter(&self) -> std::collections::btree_map::Iter<'_, K, V> { self.map.iter() }
    fn iter_mut(&mut self) -> std::collections::btree_map::IterMut<'_, K, V> { self.map.iter_mut() }
}
impl<K, V> std::ops::Index<K> for MapWrapper<K, V> where K: Ord {
    type Output = V;
    fn index(&self, index: K) -> &Self::Output { &self.map[&index] }
}
impl<K, V> std::ops::IndexMut<K> for MapWrapper<K, V> where K: Ord, V: Default {
    fn index_mut(&mut self, index: K) -> &mut Self::Output { self.map.entry(index).or_insert(Default::default()) }
}

const BIT_SIZE: usize = 128;
struct Bitset { size: usize, bits: Vec<u128> }
#[allow(dead_code)]
impl Bitset {
    fn new(size: usize) -> Self { Self { size, bits: vec![0; (size + BIT_SIZE - 1) / BIT_SIZE] } }
    fn set(&mut self, idx: usize) { assert!(idx < self.size); let (idx, bit) = (idx / BIT_SIZE, idx % BIT_SIZE); self.bits[idx] |= 1 << bit; }
    fn drop(&mut self, idx: usize) { assert!(idx < self.size); let (idx, bit) = (idx / BIT_SIZE, idx % BIT_SIZE); self.bits[idx] ^= 1 << bit; }
    fn flip(&mut self) {
        for v in self.bits.iter_mut() { *v = !*v; }
        if self.size % BIT_SIZE != 0 { let sz = self.size % BIT_SIZE; *self.bits.last_mut().unwrap() &= (1 << (sz+1)) - 1; }
    }
}
impl PartialEq for Bitset { fn eq(&self, other: &Self) -> bool { for (v, w) in self.bits.iter().zip(other.bits.iter()) { if v != w { return false; } } true } }
impl Eq for Bitset { }
impl PartialOrd for Bitset {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { self.bits.partial_cmp(&other.bits) }
    fn lt(&self, other: &Self) -> bool { for (v, w) in self.bits.iter().zip(other.bits.iter()) { if v != w { return v < w; } } false }
    fn le(&self, other: &Self) -> bool { for (v, w) in self.bits.iter().zip(other.bits.iter()) { if v != w { return v < w; } } true }
    fn gt(&self, other: &Self) -> bool { for (v, w) in self.bits.iter().zip(other.bits.iter()) { if v != w { return v > w; } } false }
    fn ge(&self, other: &Self) -> bool { for (v, w) in self.bits.iter().zip(other.bits.iter()) { if v != w { return v > w; } } true }
}
impl Ord for Bitset { fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.bits.cmp(&other.bits) } }
impl std::fmt::Debug for Bitset { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self.bits) } }

fn main() {
    input! {n: usize};

    if n == 1 {
        println!("1");
        std::process::exit(0);
    }

    let mut d = vec![-1; n+1];
    let mut cnt = 0;
    let mut prime = MapWrapper::<usize, usize>::new();
    for i in 2..n+1 {
        if d[i] >= 0 {
            continue;
        }
        prime[i] = cnt;
        cnt += 1;
        for j in 1..n+1 {
            if i * j > n {
                break;
            }
            d[i*j] = i as i32;
        }
    }

    let mut res = 0usize;
    let mut t = MapWrapper::<Bitset, usize>::new();
    for i in 1..n+1 {
        let mut now = i;
        let mut prev = 0;
        let mut p = 0;
        let mut map = vec![];

        while d[now] > 0 {
            if prev != d[now] {
                if prev != 0 {
                    map.push((prev, p));
                }
                prev = d[now];
                p = 1;
            } else {
                p += 1;
            }
            now /= d[now] as usize;
        }
        if prev != 0 {
            map.push((prev, p));
        }
        let mut bit = Bitset::new(cnt);
        for (k, v) in map {
            if v % 2 == 1 {
                bit.set(prime[k as usize]);
            }
        }
        t[bit] += 1;
    }

    for (_, v) in t.iter() {
        res += *v * *v;
    }

    println!("{}", res);
}