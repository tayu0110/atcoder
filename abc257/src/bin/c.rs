#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

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
    fn last(&self) -> Option<(&K, &V)> { if self.is_empty() { None } else { self.map.iter().next_back() } }
    fn last_mut(&mut self) -> Option<(&K, &mut V)> { if self.is_empty() { None } else { self.map.iter_mut().next() } }
    fn iter(&self) -> std::collections::btree_map::Iter<'_, K, V> { self.map.iter() }
    fn iter_mut(&mut self) -> std::collections::btree_map::IterMut<'_, K, V> { self.map.iter_mut() }
}
impl<K, V> std::ops::Index<K> for MapWrapper<K, V> where K: Ord {
    type Output = V;
    fn index(&self, index: K) -> &Self::Output { &self.map[&index] }
}
impl<K, V> std::ops::IndexMut<K> for MapWrapper<K, V> where K: Ord, V: Default {
    fn index_mut(&mut self, index: K) -> &mut Self::Output { self.map.entry(index).or_default() }
}

fn main() {
	input! {n: usize, s: Chars, w: [usize; n]};

    let mut p = w.iter().zip(s.iter()).map(|(a, b)| (*a, *b)).collect::<Vec<(usize, char)>>();
    p.sort();

    let mut map = MapWrapper::new();
    let mut cnt = 1;
    for i in 0..n {
        let (nw, _) = p[i];
        map[nw] += 1;
    }

    for (_, v) in map.iter_mut() {
        *v = cnt;
        cnt += 1;
    }

    let mut child = vec![0; cnt];
    let mut man = vec![0; cnt];

    for (nw, c) in p {
        if c == '0' {
            child[map[nw]] += 1;
        } else {
            man[map[nw]] += 1;
        }
    }
    for i in 0..cnt-1 {
        child[i+1] += child[i];
        man[i+1] += man[i];
    }

    // eprintln!("{:?}", child);
    // eprintln!("{:?}", man);

    // let csum = child[n-1];
    let msum = man[cnt-1];

    // eprintln!("{}", msum);

    let mut res = 0;
    for i in 0..cnt {
        res = std::cmp::max(child[i] + msum - man[i], res);
    }

    println!("{}", res);
}
