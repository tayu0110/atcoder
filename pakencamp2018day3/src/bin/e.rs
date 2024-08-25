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
    input! {q: usize, p: [(usize, usize, usize); q]};

    for (a, b, x) in p {
        let mut buf = vec![a, b];
        for i in 2..89 {
            let t = buf[i-1] + buf[i-2];
            if t > x {
                break;
            }
            buf.push(t);
        }

        let mut map = MapWrapper::<usize, usize>::new();
        let mut keep = MapWrapper::<usize, usize>::new();
        map[x] = 1;
        for w in buf {
            let mut tmp = MapWrapper::new();
            for (k, v) in map.iter().rev() {
                if k < &w {
                    continue;
                }
                tmp[*k - w] += *v;
            }

            for (k, v) in keep.iter() {
                map[*k] += *v;
            }

            // eprintln!("w: {}, map: {:?}, keep: {:?}", w, map.map, keep.map);
            std::mem::swap(&mut keep, &mut tmp);
        }

        for (k, v) in keep.iter() {
            map[*k] += *v;
        }

        if !map.contains(&0) {
            map[0] = 0;
        }
        println!("{}", map[0]);
    }
}
