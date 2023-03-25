#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

fn main() {
    input! {s: [Chars; 9]}

    let mut p = vec![];
    for i in 0..9 {
        for j in 0..9 {
            if s[i][j] == '#' {
                p.push((i as i64, j as i64));
            }
        }
    }

    if p.len() < 4 {
        println!("0");
        return;
    }

    let mut res = 0usize;
    let len = p.len();
    let d = |(r, c), (nr, nc)| (nr - r) * (nr - r) + (nc - c) * (nc - c);
    for i in 0..len {
        for j in i+1..len {
            for k in j+1..len {
                for l in k+1..len {
                    let mut v = geometry::convex_hull(vec![p[i], p[j], p[k], p[l]]);
                    if v.len() != 4 {
                        continue;
                    }
                    v.push(v[0]);

                    let mut prev = -1;
                    let mut bad = false;
                    for w in v.windows(2) {
                        if prev < 0 {
                            prev = d(w[0], w[1]);
                        } else {
                            if prev != d(w[0], w[1]) {
                                bad = true;
                            }
                        }
                    }

                    if d(v[0], v[2]) != d(v[1], v[3]) {
                        bad = true;
                    }

                    if !bad {
                        res += 1;
                    }
                }
            }
        }
    }

    println!("{}", res);
}

#[allow(dead_code)]
mod geometry {
    use std::ops::{ Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign, DivAssign, Neg };
    use std::convert::{ TryFrom, TryInto };
    pub trait NumericTrait 
            : Add<Self, Output = Self> + Sub<Self, Output = Self> + Mul<Self, Output = Self> + Div<Self, Output = Self> + Neg<Output = Self> 
                + AddAssign + SubAssign + MulAssign + DivAssign
                + std::fmt::Debug + std::fmt::Display + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash + Default {
        fn one() -> Self;
        fn zero() -> Self;
        fn max_value() -> Self;
        fn min_value() -> Self;
        fn abs(self) -> Self;
        fn as_float(self) -> f64;
    }
    impl NumericTrait for i64 {
        fn one() -> Self { 1 }
        fn zero() -> Self { 0 }
        fn max_value() -> Self { std::i64::MAX }
        fn min_value() -> Self { std::i64::MIN }
        fn abs(self) -> Self { self.abs() }
        fn as_float(self) -> f64 { self as f64 }
    }
    fn gcd<T: NumericTrait + Rem<Output = T>>(x: T, y: T) -> T { if y == T::zero() { x } else { gcd(y, x % y) } }
    // numerator / denominator
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct Rational {
        numerator: i64,
        denominator: i64
    }
    impl Rational {
        pub fn new(num: i64, den: i64) -> Self {
            if den == 0 { return Self { numerator: 1, denominator: 0 }; }
            if num == 0 { return Self { numerator: 0, denominator: 1 }; }
            let g = gcd(num.abs(), den.abs());
            let (numerator, denominator) = (if num / num.abs() == den / den.abs() { num.abs() } else { -num.abs() } / g, den.abs() / g);
            Self { numerator, denominator }
        }
        pub fn is_nan(&self) -> bool { self.denominator == 0 }
    }
    impl Neg for Rational { type Output = Rational; fn neg(self) -> Self::Output { assert!(!self.is_nan()); Self::new(-self.numerator, self.denominator) } }
    impl Add for Rational { type Output = Rational; fn add(self, rhs: Self) -> Self::Output { assert!(!self.is_nan()); Self::new(self.numerator * rhs.denominator + rhs.numerator * self.denominator, self.denominator * rhs.denominator) } }
    impl Sub for Rational { type Output = Rational; fn sub(self, rhs: Self) -> Self::Output { assert!(!self.is_nan()); self + (-rhs) } }
    impl Mul for Rational { type Output = Rational; fn mul(self, rhs: Self) -> Self::Output { assert!(!self.is_nan()); Self::new(self.numerator * rhs.numerator, self.denominator * rhs.denominator) } }
    impl Div for Rational { type Output = Rational; fn div(self, rhs: Self) -> Self::Output { assert!(!self.is_nan()); self * Self { numerator: rhs.denominator, denominator: rhs.numerator} } }
    impl AddAssign for Rational { fn add_assign(&mut self, rhs: Self) { assert!(!self.is_nan()); *self = self.clone() + rhs; } }
    impl SubAssign for Rational { fn sub_assign(&mut self, rhs: Self) { assert!(!self.is_nan()); *self = self.clone() - rhs; } }
    impl MulAssign for Rational { fn mul_assign(&mut self, rhs: Self) { assert!(!self.is_nan()); *self = self.clone() * rhs; } }
    impl DivAssign for Rational { fn div_assign(&mut self, rhs: Self) { assert!(!self.is_nan()); *self = self.clone() / rhs; } }
    impl std::fmt::Display for Rational { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({} / {})", self.numerator, self.denominator) } }
    impl std::fmt::Debug for Rational { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({} / {})", self.numerator, self.denominator) } }
    impl Add<f64> for Rational { type Output = f64; fn add(self, rhs: f64) -> Self::Output { let lhs: f64 = self.try_into().unwrap(); lhs + rhs } }
    impl Sub<f64> for Rational { type Output = f64; fn sub(self, rhs: f64) -> Self::Output { let lhs: f64 = self.try_into().unwrap(); lhs - rhs } }
    impl Mul<f64> for Rational { type Output = f64; fn mul(self, rhs: f64) -> Self::Output { let lhs: f64 = self.try_into().unwrap(); lhs * rhs } }
    impl Div<f64> for Rational { type Output = f64; fn div(self, rhs: f64) -> Self::Output { let lhs: f64 = self.try_into().unwrap(); lhs / rhs } }
    impl NumericTrait for Rational {
        fn one() -> Self { Self { numerator: 1, denominator: 1 } }
        fn zero() -> Self { Self { numerator: 0, denominator: 1 } }
        fn max_value() -> Self { Self { numerator: i64::max_value(), denominator: 1 } }
        fn min_value() -> Self { Self { numerator: i64::min_value(), denominator: 1 } }
        fn abs(self) -> Self { Self { numerator: self.numerator.abs(), denominator: self.denominator } }
        fn as_float(self) -> f64 { self.try_into().unwrap() }
    }
    impl<T: Into<i64>> From<T> for Rational { fn from(from: T) -> Self { Self { numerator: from.into(), denominator: 1 } } }
    impl TryInto<f64> for Rational {
        type Error = Error;
        fn try_into(self) -> Result<f64, Self::Error> { if self.is_nan() { Err(Error("Failed to convert into f64 because this is NaN.")) } else { Ok(self.numerator as f64 / self.denominator as f64) } }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    struct Vector<T: NumericTrait>(T, T);
    impl<T: NumericTrait> From<(T, T)> for Vector<T> { fn from(from: (T, T)) -> Self { Self(from.0, from.1) } }
    impl<T: NumericTrait> From<[T; 2]> for Vector<T> { fn from(from: [T; 2]) -> Self { Self(from[0], from[1]) } }
    impl<T: NumericTrait> TryFrom<Vec<T>> for Vector<T> {
        type Error = Error;
        fn try_from(value: Vec<T>) -> Result<Self, Self::Error> { match value.len() { 2 => Ok(Self(value[0], value[1])), _ => Err(Error("Failed to generate the instance of geometry::Vector because the length of the argument Vec<i64> is not 2.")) } }
    }
    impl<T: NumericTrait> Vector<T> {
        fn new(from: [T; 2], to: [T; 2]) -> Self { Self(to[0] - from[0], to[1] - from[1]) }
        fn inner_product(&self, rhs: &Vector<T>) -> T { self.0 * rhs.0 + self.1 * rhs.1 }
        fn outer_product(&self, rhs: &Vector<T>) -> T { self.0 * rhs.1 - self.1 * rhs.0 }
        fn scalar_product(&self, rhs: T) -> Self { Self(self.0 * rhs, self.1 * rhs) }
        fn is_vertical(&self, rhs: &Vector<T>) -> bool { self.inner_product(rhs) == T::zero() }
        fn is_parallel(&self, rhs: &Vector<T>) -> bool { self.outer_product(rhs) == T::zero() }
        // 0 <= theta <= 180
        fn arg(&self, rhs: &Vector<T>) -> f64 { ((self.0*rhs.0 + self.1*rhs.1).as_float() / ((self.0*self.0 + self.1*self.1) * (rhs.0*rhs.0 + rhs.1*rhs.1)).as_float().sqrt()).acos() }
    }
    impl<T: NumericTrait> Add for Vector<T> { type Output = Vector<T>; fn add(self, rhs: Self) -> Self::Output { Self(self.0 + rhs.0, self.1 + rhs.1) } }
    impl<T: NumericTrait> Sub for Vector<T> { type Output = Vector<T>; fn sub(self, rhs: Self) -> Self::Output { Self(self.0 - rhs.0, self.1 - rhs.1) } }
    impl<T: NumericTrait> Neg for Vector<T> { type Output = Vector<T>; fn neg(self) -> Self::Output { Self(-self.0, -self.1) } }
    impl<T: NumericTrait> AddAssign for Vector<T> { fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; self.1 += rhs.1; } }
    impl<T: NumericTrait> SubAssign for Vector<T> { fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0; self.1 -= rhs.1; } }
    #[derive(Debug)]
    pub struct Error(&'static str);
    // 凸包の構成点をx座標が最も小さいものから時計回りに返す
    pub fn convex_hull<T: NumericTrait>(mut points: Vec<(T, T)>) -> Vec<(T, T)> {
        if points.len() < 2 { return points; }
        points.sort();
        let mut convex = vec![vec![]; 2];
        let check = [std::cmp::Ordering::Less, std::cmp::Ordering::Greater];
        for (x, y) in points {
            for (i, convex) in convex.iter_mut().enumerate() {
                while convex.len() >= 2 {
                    let ((sx, sy), (fx, fy)) = (convex.pop().unwrap(), *convex.last().unwrap());
                    let (f, s) = (Vector::new([fx, fy], [sx, sy]), Vector::new([sx, sy], [x, y]));
                    let outer_product = f.outer_product(&s);
                    if outer_product.cmp(&T::zero()) == check[i] || outer_product == T::zero() {
                        convex.push((sx, sy));
                        break;
                    }
                }
                convex.push((x, y));
            }
        }
        convex.into_iter().enumerate().flat_map(|(i, mut v)| if i == 0 { v } else { v.reverse(); let len = v.len(); v[1..len-1].into() }).collect()
    }
    // points(周上の順番であることが必要)に含まれる点を結んだ線を周とする多角形の面積求める
    pub fn points_to_area<T: NumericTrait>(points: &Vec<(T, T)>) -> T {
        let len = points.len();
        let mut res = T::zero();
        for (i, (x, y)) in points.into_iter().enumerate() {
            let (nx, ny) = points[(i+1) % len];
            res += (*x - nx) * (*y + ny);
        }
        res.abs() / (T::one() + T::one())
    }
    pub fn sort_by_arg<T: NumericTrait>(mut points: Vec<(T, T)>) -> Vec<(T, T)> {
        points.sort_by(|&(x0, y0), &(x1, y1)| {
            ((y0, x0) < (T::zero(), T::zero())).cmp(&((y1, x1) < (T::zero(), T::zero()))).then_with(|| (x1 * y0).cmp(&(x0 * y1)))
        });
        points
    }
}
