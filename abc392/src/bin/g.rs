use std::sync::Mutex;

use convolution::convolution;
use static_modint::Mod998244353;

static COUNT: Mutex<[u8; 1000001]> = Mutex::new([0; 1000001]);

fn main() {
    cpio::scan!(n: usize, mut s: [u32; n]);

    let mut pol = COUNT.lock().unwrap();
    let mut max = 0;
    for &s in &s {
        pol[s as usize] = 1;
        max = max.max(s);
    }

    let mut poly = Vec::with_capacity(2000001);
    poly.extend(pol.iter().take(max as usize + 1).copied().map(|p| p as u32));
    let res = convolution::<Mod998244353>(poly.clone(), poly);
    let mut sum = 0;
    for b2 in s {
        sum += res[(b2 << 1) as usize] as usize;
    }

    println!("{}", (sum - n) / 2)
}
