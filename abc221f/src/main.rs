use proconio::input;

const MOD: i64 = 998244353;
#[derive(Clone, Copy, PartialEq, Eq)]
struct Mint {
    val: i64,
    modulo: i64
}
#[allow(dead_code)]
impl Mint {
    fn new(val: i64) -> Self { let modulo = MOD; Mint { val: val % modulo, modulo } }
    fn val(&self) -> i64 { self.val % self.modulo }
    fn pow(&self, mut exp: i64) -> Self {
        let mut val = self.val;
        let mut res = 1;
        while exp > 0 {
            if exp % 2 == 1 {
                res = (res * val) % self.modulo;
            }
            val = (val * val) % self.modulo;
            exp /= 2;
        }
        Self { val: res, modulo: self.modulo }
    }
    fn inv(&self) -> Self { self.pow(self.modulo - 2) }
}
impl Default for Mint { fn default() -> Self { Mint { val: 0, modulo: MOD }}}
impl std::fmt::Debug for Mint { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.val) } }
impl std::fmt::Display for Mint { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.val) } }
impl std::ops::Add for Mint { type Output = Self; fn add(self, rhs: Self) -> Self::Output { Self::new(self.val + rhs.val) } }
impl std::ops::AddAssign for Mint { fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; } }
impl std::ops::Sub for Mint { type Output = Self; fn sub(self, rhs: Self) -> Self::Output { Self::new(self.val - rhs.val + self.modulo) } }
impl std::ops::SubAssign for Mint { fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; } }
impl std::ops::Mul for Mint { type Output = Self; fn mul(self, rhs: Self) -> Self::Output { Self::new(self.val * rhs.val) } }
impl std::ops::MulAssign for Mint { fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; } }
impl std::ops::Div for Mint { type Output = Self; fn div(self, rhs: Self) -> Self::Output { if rhs.val == 0 { panic!("0 division"); } else { self * rhs.inv() } } }
impl std::ops::DivAssign for Mint { fn div_assign(&mut self, rhs: Self) { if rhs.val == 0 { panic!("0 division"); } else { *self *= rhs.inv() } } }
struct Combination {
    fact: Vec<Mint>,
    ifact: Vec<Mint>
}
impl Combination {
    #[allow(dead_code)]
    fn new(size: usize) -> Self {
        let mut fact = vec![Mint::new(0); size+1];
        let mut ifact = vec![Mint::new(0); size+1];
        fact[0] = Mint::new(1);
        ifact[0] = Mint::new(1);
        for v in 1..size+1 {
            fact[v] = Mint::new(fact[v-1].val) * Mint::new(v as i64);
        }
        ifact[size] = fact[size].inv();
        for v in (1..size).rev() {
            ifact[v] = ifact[v+1] * Mint::new(v as i64 + 1) ;
        }
        Self { fact, ifact }
    }
    #[allow(dead_code)]
    fn get(&self, n: usize, k: usize) -> Mint {
        if n < k {
            Mint::new(0)
        } else {
            self.fact[n] * self.ifact[k] * self.ifact[n-k]
        }
    }
}

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

fn dfs(now: usize, par: usize, d: i32, dist: &mut Vec<i32>, t: &Vec<Vec<usize>>) {
    if dist[now] >= 0 {
        return;
    }
    dist[now] = d;
    for to in &t[now] {
        if par == *to || dist[*to] >= 0 {
            continue;
        }
        dfs(*to, now, d+1, dist, t);
    }
}

fn dfs2(now: usize, par: usize, d: i32, ck: &mut Vec<(usize, i32)>, t: &Vec<Vec<usize>>) {
    ck.push((now, d));
    for to in &t[now] {
        if *to == par {
            continue;
        }
        dfs2(*to, now, d+1, ck, t);
    }
}

fn main() {
    input! {n: usize, p: [(usize, usize); n-1]};

    let mut t = vec![vec![]; n];
    for (u, v) in p {
        t[u-1].push(v-1);
        t[v-1].push(u-1);
    }

    let mut tmp = vec![-1; n];
    dfs(0, 1001001001001, 0, &mut tmp, &t);

    let root = tmp.binary_search(tmp.iter().max().unwrap()).unwrap();
    let mut dist = vec![-1; n];
    dfs(root, 1001001001001, 0, &mut dist, &t);

    let d = dist.iter().max().unwrap();
    
    let root = {
        let mut res = 0;
        for (i, v) in dist.iter().enumerate() {
            if *v == *d / 2 && t[i].len() > 1 {
                res = i;
                break;
            }
        }
        res
    };

    let mut dist = vec![vec![]; *d as usize + 1];
    dist[0].push(1);
    for child in &t[root] {
        let mut ck = vec![];
        dfs2(*child, root, 0, &mut ck, &t);
        eprintln!("child: {}, ck: {:?}", child, ck);
        let mut map: MapWrapper<i32, usize> = MapWrapper::<i32, usize>::new();
        for (_, d) in &ck {
            map[*d + 1] += 1;
        }
        for (key, val) in map.iter() {
            dist[*key as usize].push(*val);
        }
    }
    eprintln!("dist: {:?}", dist);

    let mut res = Mint::new(0);
    for (i, v) in dist.iter().enumerate() {
        if dist[*d as usize - i].len() == 0 {
            continue;
        }
        if *d as usize - i == i {
            let mut dp = vec![Mint::new(0); v.len()+1];
            dp[0] = Mint::new(1);
            for k in v {
                for j in (0..dp.len()).rev() {
                    if dp[j] == Mint::new(0) {
                        continue;
                    }
                    let tmp = dp[j] * Mint::new(*k as i64);
                    dp[j+1] += tmp;
                }
            }
            for j in 2..dp.len() {
                res += dp[j];
            }
        } else {
            let mut dp = vec![Mint::new(0); dist[*d as usize - i].len() + 2];
            dp[1] += v.iter().fold(Mint::new(0), |sum, x| sum + Mint::new(*x as i64));
            for k in &dist[*d as usize - i] {
                for j in (1..dp.len()).rev() {
                    if dp[j] == Mint::new(0) {
                        continue;
                    }
                    let tmp = dp[j] * Mint::new(*k as i64);
                    dp[j+1] += tmp;
                }
            }
            for j in 2..dp.len() {
                res += dp[j];
            }
            if *d as usize - i < i {
                res -= dp[2];
            }
        }
    }

    println!("{}", res);
}
