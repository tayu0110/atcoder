use proconio::input;

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
    fn test(&self, idx: usize) -> bool { (self.bits[idx / BIT_SIZE] & (1u128 << (idx % BIT_SIZE))) != 0 }
    fn count_ones(&self) -> usize { let mut res = 0; for v in self.bits.iter() { res += v.count_ones() as usize; } res }
    fn bitwise_and(&self, rhs: &Bitset) -> Self {
        let mut bits = vec![];
        for (v, w) in self.bits.iter().zip(rhs.bits.iter()) { bits.push(v & w); }
        Self { size: self.size, bits }
    }
    fn bitwise_or(&self, rhs: &Bitset) -> Bitset {
        let mut bits = vec![];
        for (v, w) in self.bits.iter().zip(rhs.bits.iter()) { bits.push(v | w); }
        Self { size: self.size, bits }
    }
    fn bitwise_xor(&self, rhs: &Bitset) -> Self {
        let mut bits = vec![];
        for (v, w) in self.bits.iter().zip(rhs.bits.iter()) { bits.push(v ^ w); }
        Self { size: self.size, bits }
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
impl Clone for Bitset {
    fn clone(&self) -> Self {
        let mut bits = vec![];
        for v in &self.bits {
            bits.push(*v);
        }
        Self { size: self.size, bits }
    }
}
impl std::fmt::Debug for Bitset { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self.bits) } }

fn main() {
    input! {n: usize, p: [(usize, usize, usize); n]};

    const MAX: usize = 5010;

    let mut p = p.into_iter().filter(|&(d, c, _)| c <= d).map(|(d, c, s)| (d, d - c, c, s)).collect::<Vec<_>>();
    p.sort();
    let mut dp = vec![0usize; MAX];

    for (end, _, span, s) in p {
        for now in (0..MAX).rev() {
            if now + span > end {
                continue;
            }
            dp[now+span] = std::cmp::max(dp[now+span], dp[now] + s);
        }
    }

    println!("{}", dp.into_iter().max().unwrap());
}