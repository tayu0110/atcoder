use proconio::input;

#[allow(dead_code)]
mod convolution {
    const COMPLEX_ONE: num::Complex<f64> = num::Complex::new(1f64, 0f64);
    const PI2: f64 = std::f64::consts::PI * 2f64;
    fn fft(a: &mut Vec<num::Complex<f64>>, inv: bool) {
        let n = a.len();
        let mut r = num::Complex::from_polar(&1f64, &(PI2 / n as f64));
        if inv { r = r.inv(); }
        let b = &mut vec![num::Complex::default(); n];
        for i in (0..n.trailing_zeros()).map(|v| 1usize << v).rev() {
            let z = r.powu(i as u32);
            let mut z2 = COMPLEX_ONE;
            for j in (0..n).step_by(i*2) {
                for k in 0..i {
                    a[i+j+k] *= z2;
                    b[j/2+k] = a[j+k] + a[i+j+k];
                    b[n/2+j/2+k] =a[j+k] - a[i+j+k];
                }
                z2 *= z;
            }
            std::mem::swap(a, b);
        }
        if inv { a.iter_mut().for_each(|now| *now /= num::Complex::new(n as f64, 0f64)); }
    }
    pub fn fconvolution(a: &Vec<f64>, b: &Vec<f64>) -> Vec<f64> {
        let deg = a.len() + b.len() - 1;
        let mut n = 1;
        while n < deg { n <<= 1; }
        let mut a2 = vec![num::Complex::default(); n];
        a.into_iter().enumerate().for_each(|(i, v)| a2[i] = num::Complex::new(*v, 0f64));
        let mut b2 = vec![num::Complex::default(); n];
        b.into_iter().enumerate().for_each(|(i, v)| b2[i] = num::Complex::new(*v, 0f64));
        fft(&mut a2, false);
        fft(&mut b2, false);
        let mut c2 = a2.into_iter().zip(b2.into_iter()).map(|(l, r)| l * r).collect::<Vec<_>>();
        fft(&mut c2, true);
        c2[0..deg].into_iter().map(|v| v.re).collect()
    }
    pub fn iconvolution(a: &Vec<i64>, b: &Vec<i64>) -> Vec<i64> {
        let a = a.iter().map(|v| *v as f64).collect::<Vec<_>>();
        let b = b.iter().map(|v| *v as f64).collect::<Vec<_>>();
        fconvolution(&a, &b).into_iter().map(|v| v.round() as i64).collect()
    }
}

fn main() {
    input! {n: usize, p: [(i64, i64); n]};

    let (a, b) = p.into_iter().fold((vec![0], vec![0]), |mut vv, (l, r)| {
        vv.0.push(l);
        vv.1.push(r);
        vv
    });

    let c = convolution::iconvolution(&a, &b);
    for v in c.into_iter().skip(1) {
        println!("{}", v);
    }
}