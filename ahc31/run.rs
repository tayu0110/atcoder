pub use __cargo_equip::prelude::*;

use math::factorize;
use proconio::*;

fn main() {
    input! {w: u64, d: usize, n: usize, a: [[u64; n]; d]}

    for a in a {
        let mut sum = a.iter().sum::<u64>();
        let mut a = a.into_iter().enumerate().collect::<Vec<_>>();
        a.sort_unstable_by_key(|v| v.1);

        let mut res = vec![(0, 0, 0, 0); n];
        let mut t = vec![(w as u64, w as u64, 0, 0)];
        let mut rem = w * w;
        while let Some((i, a)) = a.pop() {
            sum -= a;
            let mut factor = factorize(a as u64);
            factor.sort_unstable();

            let mut diff = u64::MAX;
            let mut j = None;
            for k in 0..t.len() {
                let (h, w, _, _) = t[k];
                if h * w < a {
                    continue;
                }

                {
                    let fp = factor.partition_point(|&f| f <= h);
                    if fp > 0
                        && factor[fp - 1] <= h
                        && a / factor[fp - 1] <= w
                        && h - factor[fp - 1] < diff
                    {
                        diff = h - factor[fp - 1];
                        j = Some((k, true));
                    }
                }
                {
                    let fp = factor.partition_point(|&f| f <= w);
                    if fp > 0
                        && factor[fp - 1] <= w
                        && a / factor[fp - 1] <= h
                        && w - factor[fp - 1] < diff
                    {
                        diff = w - factor[fp - 1];
                        j = Some((k, false));
                    }
                }
            }

            if let Some((j, f)) = j {
                if f {
                    let (h, w, r, c) = t.swap_remove(j);
                    let fp = factor.partition_point(|&f| f <= h) - 1;
                    let f = factor[fp];
                    let g = a / f;
                    res[i] = (r, c, r + f, c + g);
                    rem -= a;
                    if h != f {
                        t.push((h - f, w, r + f, c));
                    }
                    if w != g {
                        t.push((f, w - g, r, c + g));
                    }
                } else {
                    let (h, w, r, c) = t.swap_remove(j);
                    let fp = factor.partition_point(|&f| f <= w) - 1;
                    let g = factor[fp];
                    let f = a / g;
                    rem -= a;
                    res[i] = (r, c, r + f, c + g);
                    if h != f {
                        t.push((h - f, g, r + f, c));
                    }
                    if w != g {
                        t.push((h, w - g, r, c + g));
                    }
                }
            } else {
                let max = t.iter().map(|(h, w, _, _)| h * w).max().unwrap();
                let pos = t.iter().position(|(h, w, _, _)| h * w == max).unwrap();
                let (h, w, r, c) = t.swap_remove(pos);
                let rhh = h * ((a + h - 1) / h).min(w).max(1);
                let rhw = w * ((a + w - 1) / w).min(h).max(1);
                if rhh.abs_diff(a) < rhw.abs_diff(a) && rem >= rhh + sum {
                    res[i] = (r, c, r + h, c + ((a + h - 1) / h).min(w).max(1));
                    if w > (a + h - 1) / h {
                        t.push((h, w - (a + h - 1) / h, r, c + (a + h - 1) / h));
                    }
                    rem -= rhh;
                } else if rhh.abs_diff(a) > rhw.abs_diff(a) && rem >= rhw + sum {
                    res[i] = (r, c, r + ((a + w - 1) / w).min(h).max(1), c + w);
                    if h > a / w {
                        t.push((h - (a + w - 1) / w, w, r + (a + w - 1) / w, c));
                    }
                    rem -= rhw;
                } else if a - h * (a / h).min(w).max(1) < a - w * (a / w).min(h).max(1) {
                    res[i] = (r, c, r + h, c + (a / h).min(w).max(1));
                    if w > a / h {
                        t.push((h, w - a / h, r, c + a / h));
                    }
                    rem -= h * (a / h).min(w).max(1);
                } else {
                    res[i] = (r, c, r + (a / w).min(h).max(1), c + w);
                    if h > a / w {
                        t.push((h - a / w, w, r + a / w, c));
                    }
                    rem -= w * (a / w).min(h).max(1);
                }
            }
        }

        for (r, c, h, w) in res {
            println!("{r} {c} {h} {w}");
        }
    }
}

// The following code was expanded by `cargo-equip`.

///  # Bundled libraries
/// 
///  - `arbitrary-montgomery-modint 0.1.0 (git+https://github.com/tayu0110/tayu-procon.git#8f9c90a28cfad37b51b646369aa00abcebfe3e68)` licensed under `CC0-1.0` as `crate::__cargo_equip::crates::__arbitrary_montgomery_modint_0_1_0`
///  - `math 0.1.0 (git+https://github.com/tayu0110/tayu-procon.git#8f9c90a28cfad37b51b646369aa00abcebfe3e68)`                        licensed under `CC0-1.0` as `crate::__cargo_equip::crates::math`
///  - `numeric 0.1.0 (git+https://github.com/tayu0110/tayu-procon.git#8f9c90a28cfad37b51b646369aa00abcebfe3e68)`                     licensed under `CC0-1.0` as `crate::__cargo_equip::crates::__numeric_0_1_0`
///  - `simple-rand 0.1.0 (git+https://github.com/tayu0110/tayu-procon.git#8f9c90a28cfad37b51b646369aa00abcebfe3e68)`                 licensed under `CC0-1.0` as `crate::__cargo_equip::crates::__simple_rand_0_1_0`
///  - `zero-one 0.1.0 (git+https://github.com/tayu0110/tayu-procon.git#8f9c90a28cfad37b51b646369aa00abcebfe3e68)`                    licensed under `CC0-1.0` as `crate::__cargo_equip::crates::__zero_one_0_1_0`
#[cfg_attr(any(), rustfmt::skip)]
#[allow(unused)]
mod __cargo_equip {
    pub(crate) mod crates {
        pub mod __arbitrary_montgomery_modint_0_1_0 {use std::ops::{Add,AddAssign,Div,DivAssign,Mul,MulAssign,Sub,SubAssign};const fn montgomery_reduction(val:u64,modulo:u64,modulo_inv:u64)->u64{let(t,f)=(((val.wrapping_mul(modulo_inv)as u128).wrapping_mul(modulo as u128)>>64)as u64).overflowing_neg();t.wrapping_add(modulo*f as u64)}const fn montgomery_multiplication(lhs:u64,rhs:u64,modulo:u64,modulo_inv:u64)->u64{let a=lhs as u128*rhs as u128;let(t,f)=((a>>64)as u64).overflowing_sub((((a as u64).wrapping_mul(modulo_inv)as u128).wrapping_mul(modulo as u128)>>64)as u64,);t.wrapping_add(modulo*f as u64)}#[derive(Clone,Copy,PartialEq,Eq,Hash)]pub struct ArbitraryMontgomeryModint{pub val:u64,modulo:u64,modulo_inv:u64,r:u64,r2:u64,}impl ArbitraryMontgomeryModint{#[inline]pub const fn new(val:u64,modulo:u64)->Self{Self::raw(val%modulo,modulo)}pub const fn raw(val:u64,modulo:u64)->Self{if modulo==998244353{let val=montgomery_multiplication(val,299560064,modulo,996491785301655553);return Self{val,modulo,modulo_inv:996491785301655553,r:932051910,r2:299560064,};}let r=((1u128<<64)%modulo as u128)as u64;let r2=((modulo as u128).wrapping_neg()%modulo as u128)as u64;let modulo_inv={let inv=modulo.wrapping_mul(2u64.wrapping_sub(modulo.wrapping_mul(modulo)));let inv=inv.wrapping_mul(2u64.wrapping_sub(modulo.wrapping_mul(inv)));let inv=inv.wrapping_mul(2u64.wrapping_sub(modulo.wrapping_mul(inv)));let inv=inv.wrapping_mul(2u64.wrapping_sub(modulo.wrapping_mul(inv)));inv.wrapping_mul(2u64.wrapping_sub(modulo.wrapping_mul(inv)))};let val=montgomery_multiplication(val,r2,modulo,modulo_inv);Self{val,modulo,modulo_inv,r,r2}}#[inline]const fn from_raw_parts_unchecked(val:u64,modulo:u64,modulo_inv:u64,r:u64,r2:u64,)->Self{Self{val,modulo,modulo_inv,r,r2}}#[inline]pub const fn from_same_mod(val:u64,from:Self)->Self{Self::from_same_mod_unchecked(val%from.modulo,from)}#[inline]pub const fn from_same_mod_unchecked(val:u64,from:Self)->Self{let val=montgomery_multiplication(val,from.r2,from.modulo,from.modulo_inv);Self::from_raw_parts_unchecked(val,from.modulo,from.modulo_inv,from.r,from.r2)}#[inline]pub const fn val(&self)->u64{montgomery_reduction(self.val,self.modulo,self.modulo_inv)}#[inline]pub const fn rawval(&self)->u64{self.val}#[inline]pub const fn one(&self)->Self{Self{val:self.r,modulo:self.modulo,modulo_inv:self.modulo_inv,r:self.r,r2:self.r2,}}#[inline]pub const fn zero(&self)->Self{Self{val:0,modulo:self.modulo,modulo_inv:self.modulo_inv,r:self.r,r2:self.r2,}}pub fn pow(&self,mut n:u64)->Self{let mut val=self.val;let mut res=self.r;while n!=0{if n&1!=0{res=montgomery_multiplication(res,val,self.modulo,self.modulo_inv);}val=montgomery_multiplication(val,val,self.modulo,self.modulo_inv);n>>=1;}Self{val:res,modulo:self.modulo,modulo_inv:self.modulo_inv,r:self.r,r2:self.r2,}}#[inline]pub fn inv(&self)->Self{self.pow(self.modulo-2)}}impl Add for ArbitraryMontgomeryModint{type Output=Self;fn add(self,rhs:Self)->Self::Output{let(t,fa)=self.val.overflowing_add(rhs.val);let(u,fs)=t.overflowing_sub(self.modulo);Self{val:if fa||!fs{u}else{t},modulo:self.modulo,modulo_inv:self.modulo_inv,r:self.r,r2:self.r2,}}}impl Sub for ArbitraryMontgomeryModint{type Output=Self;fn sub(self,rhs:Self)->Self::Output{let(val,f)=self.val.overflowing_sub(rhs.val);Self{val:if f{val.wrapping_add(self.modulo)}else{val},modulo:self.modulo,modulo_inv:self.modulo_inv,r:self.r,r2:self.r2,}}}impl Mul for ArbitraryMontgomeryModint{type Output=Self;fn mul(self,rhs:Self)->Self::Output{Self{val:montgomery_multiplication(self.val,rhs.val,self.modulo,self.modulo_inv),modulo:self.modulo,modulo_inv:self.modulo_inv,r:self.r,r2:self.r2,}}}impl Div for ArbitraryMontgomeryModint{type Output=Self;fn div(self,rhs:Self)->Self::Output{self*rhs.inv()}}impl AddAssign for ArbitraryMontgomeryModint{fn add_assign(&mut self,rhs:Self){*self=*self+rhs;}}impl SubAssign for ArbitraryMontgomeryModint{fn sub_assign(&mut self,rhs:Self){*self=*self-rhs;}}impl MulAssign for ArbitraryMontgomeryModint{fn mul_assign(&mut self,rhs:Self){*self=*self*rhs;}}impl DivAssign for ArbitraryMontgomeryModint{fn div_assign(&mut self,rhs:Self){*self=*self/rhs;}}impl std::fmt::Debug for ArbitraryMontgomeryModint{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"{}",self.val())}}impl std::fmt::Display for ArbitraryMontgomeryModint{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"{}",self.val())}}}
        pub mod math {use crate::__cargo_equip::preludes::math::*;use arbitrary_montgomery_modint::ArbitraryMontgomeryModint;use numeric::Integer;use simple_rand::xor_shift;#[inline]pub fn gcd<T:Integer>(mut x:T,mut y:T)->T{while y!=T::zero(){let(nx,ny)=(y,x%y);x=nx;y=ny;}x}#[inline]pub fn lcm<T:Integer>(x:T,y:T)->T{x/gcd(x,y)*y}#[inline]pub fn ext_gcd<T:Integer>(a:T,b:T)->(T,T,T){let(mut s,mut t)=(a,b);let(mut sx,mut tx)=(T::one(),T::zero());let(mut sy,mut ty)=(T::zero(),T::one());while s%t!=T::zero(){let d=s/t;let u=s%t;let ux=sx-tx*d;let uy=sy-ty*d;s=t;sx=tx;sy=ty;t=u;tx=ux;ty=uy;}(t,tx,ty)}#[inline]pub fn mod_pow(a:i64,mut n:i64,p:i64)->i64{let mut res=1;let mut pow=a;while n!=0{if n&1!=0{res=(res as i128*pow as i128%p as i128)as i64;}pow=(pow as i128*pow as i128%p as i128)as i64;n>>=1;}res}#[inline]pub fn mod_log(a:i64,b:i64,p:i64)->Option<i64>{mod_log_with_lower_bound_constraint(a,b,p,0)}pub fn mod_log_with_lower_bound_constraint(a:i64,b:i64,p:i64,lower:i64)->Option<i64>{let(a,b)=(a.rem_euclid(p),b.rem_euclid(p));if p==1{return Some(lower);}if b==1&&lower<=0{return Some(0);}let(g,inv,_)=ext_gcd(a,p);if g!=1{if b%g!=0{return None;}let(na,nb,np)=(a/g,b/g,p/g);let(_,inv,_)=ext_gcd(na,np);let inv=inv.rem_euclid(np);if let Some(res)=mod_log(a,nb*inv,np){return Some(res+1);}else{return None;}}let m=(p as f64).sqrt().ceil()as i64;assert!(m*m>=p);let mut now=1;let mut map=std::collections::HashMap::new();for j in 0..m{map.entry(now).or_insert(vec![]).push(j);now=(now as i128*a as i128%p as i128)as i64;}let inv=mod_pow(inv.rem_euclid(p),m,p);debug_assert_eq!((now as i128*inv as i128).rem_euclid(p as i128),1);let mut now=1;for i in 0..=m{let r=(b as i128*now as i128%p as i128)as i64;if let Some(v)=map.get(&r){for j in v{if i*m+j<lower{continue;}return Some(i*m+j);}}now=(now as i128*inv as i128%p as i128)as i64;}None}pub fn baby_step_giant_step(a:i64,b:i64,p:i64,f:impl Fn(i64,i64)->i64,f_inv:impl Fn(i64,i64)->i64,)->Option<i64>{let m=(p as f64).sqrt().ceil()as i64;assert!(m*m>=p);let mut map=std::collections::HashMap::new();for j in 0..=m{let now=f(a,j);map.entry(now).or_insert(j);}let mut now=f_inv(b,0);for i in 0..=m{if let Some(j)=map.get(&now){return Some(i*m+j);}now=f_inv(now,m);}None}#[inline]pub fn chinese_remainder_theorem(mut a:i64,mut m1:i64,mut b:i64,mut m2:i64,)->Option<(i64,i64)>{if m1<m2{std::mem::swap(&mut a,&mut b);std::mem::swap(&mut m1,&mut m2);}let(a,b)=(a.rem_euclid(m1),b.rem_euclid(m2));if m1%m2==0{return if a%m2!=b{None}else{Some((a,m1))};}let(d,k,_)=ext_gcd(m1,m2);let u1=m2/d;if a%d!=b%d{return None;}let x=(b-a)/d%u1*k%u1;let m=m1*u1;let res=(a+x*m1).rem_euclid(m);Some((res,m))}#[inline]pub fn garner(a:&[i64],p:&[i64],modulo:i64)->(i64,i64){assert_eq!(a.len(),p.len());let mut prod=vec![1;p.len()+1];let mut res=vec![0;p.len()+1];for(i,(&a,&m))in a.iter().zip(p.iter()).enumerate(){let a=a%m;let(_,inv,_)=ext_gcd(prod[i],m);let t=((a-res[i])*inv).rem_euclid(m);for(i,&p)in p.iter().enumerate().skip(i+1){res[i]=(res[i]+(t*prod[i]))%p;prod[i]=(prod[i]*m)%p;}res[p.len()]=(res[p.len()]+(t*prod[p.len()]))%modulo;prod[p.len()]=(prod[p.len()]*m)%modulo;}(res[p.len()],prod[p.len()])}#[inline]pub fn garner_prechecked(a:&[i64],p:&[i64],modulo:i64)->Option<(i64,i64)>{let mut p=p.to_vec();for i in 0..a.len(){for j in 0..i{let mut g=gcd(p[i],p[j]);if(a[i]-a[j])%g!=0{return None;}p[i]/=g;p[j]/=g;let mut gi=gcd(p[i],g);let mut gj=g/gi;g=gcd(gi,gj);gi*=g;gj/=g;while g!=1{g=gcd(gi,gj);gi*=g;gj/=g;}p[i]*=gi;p[j]*=gj;}}Some(garner(a,&p,modulo))}pub fn miller_rabin_test(p:u64)->bool{if p==1||p&1==0{return p==2;}let s=(p-1).trailing_zeros();let t=(p-1)>>s;let mont_zero=ArbitraryMontgomeryModint::raw(0,p);let mont_one=mont_zero.one();let mont_neg_one=mont_zero-mont_one;[2,325,9375,28178,450775,9780504,1795265022].iter().map(|&a|a%p).filter(|&a|a!=0).all(|a|{let a=ArbitraryMontgomeryModint::from_same_mod_unchecked(a,mont_zero);let at=a.pow(t);if at==mont_one||at==mont_neg_one{return true;}(1..s).scan(at,|at,_|{*at*=*at;Some(*at)}).any(|at|at==mont_neg_one)})}pub fn divisors_enumeration(n:u64)->Vec<u64>{let mut f=factorize(n);f.sort();let mut t=vec![];for f in f{match t.last_mut(){Some((c,cnt))if*c==f=>*cnt+=1,_=>t.push((f,1)),}}let mut res=vec![1];for(c,cnt)in t{let mut now=1;let len=res.len();for _ in 0..cnt{now*=c;for i in 0..len{let new=res[i]*now;res.push(new);}}}res}pub fn factorize(mut n:u64)->Vec<u64>{if n==1{return vec![];}let mut res=vec![2u64;n.trailing_zeros()as usize];n>>=n.trailing_zeros();while let Some(g)=pollard_rho(n){while n%g==0{res.push(g);n/=g;}}if n>1{res.push(n);}res}fn pollard_rho(n:u64)->Option<u64>{if n<=1{return None;}if n&1==0{return Some(2);}if miller_rabin_test(n){return Some(n);}let m=(n as f64).powf(0.125).round()as i64+1;let mont_zero=ArbitraryMontgomeryModint::raw(0,n);let mont_one=mont_zero.one();let mut g=1;for c in(1..n).map(|c|ArbitraryMontgomeryModint::from_same_mod_unchecked(c,mont_zero)){let mut y=mont_zero;let mut q=mont_one;'base:for r in(0..).map(|i|1<<i){let x=y;(r..=(3*r)>>2).for_each(|_|y=y*y+c);for k in(((3*r)>>2)..r).step_by(m as usize){let ys=y;(0..std::cmp::min(m,r-k)).for_each(|_|{y=y*y+c;q*=x-y;});g=gcd(q.val()as i64,n as i64);if g!=1{if g==n as i64{y=ys*ys+c;while gcd((x-y).val()as i64,n as i64)==1{y=y*y+c;}g=gcd((x-y).val()as i64,n as i64);}break 'base;}}}if g!=n as i64{break;}}pollard_rho(g as u64)}pub fn tonelli_shanks_unchecked(n:u64,p:u64)->Option<u64>{if p==2{let res=n&1;assert_eq!(res*res%p,n);return Some(res);}type Modint=ArbitraryMontgomeryModint;let mn=Modint::new(n,p);if mn.rawval()==0{assert_eq!(0%p,n);return Some(0);}let one=mn.one();if mn.pow((p-1)>>1).rawval()!=one.rawval(){return None;}if p&0b11==3{let s=mn.pow((p+1)>>2).val();let t=p-s;return Some(s.min(t));}for b in xor_shift(381928476372819).map(|v|v%(p-2)+2){let b=Modint::from_same_mod(b,mn);if b.pow((p-1)>>1).rawval()!=one.rawval(){let q=(p-1).trailing_zeros()as u64;let s=(p-1)>>q;let mut x=mn.pow((s+1)>>1);let mut x2=x*x;let mut b=b.pow(s);let mninv=mn.inv();let mut shift=2;while x2!=mn{let diff=mninv*x2;if diff.pow(1<<(q-shift)).rawval()!=one.rawval(){x*=b;b*=b;x2*=b;}else{b*=b;}shift+=1;}return Some(x.val());}}unreachable!()}pub fn tonelli_shanks(n:u64,p:u64)->Option<u64>{assert!(miller_rabin_test(p));tonelli_shanks_unchecked(n,p)}pub fn xor_base(a:&[u64])->Vec<u64>{let mut res=vec![];for&(mut a)in a{for&base in&res{a=a.min(a^base);}if a>0{res.push(a);}}res}pub struct Sieve<const MAX:usize=1000010>{count:usize,primes:[usize;MAX],}impl<const MAX:usize>Sieve<MAX>{pub const fn new()->Self{let mut primes=[usize::MAX;MAX];let mut count=0;let mut i=2;while i<MAX{if primes[i]==usize::MAX{primes[i]=i;primes[count]=i;count+=1;}let mut j=0;while j<count{if primes[j]>primes[i]||primes[j]*i>=MAX{break;}primes[primes[j]*i]=primes[j];j+=1;}i+=1;}Self{count,primes}}pub const fn count(&self)->usize{self.count}}impl std::ops::Index<usize>for Sieve{type Output=usize;fn index(&self,index:usize)->&Self::Output{&self.primes[index]}}pub fn primitive_root(p:u64)->u64{if p==2{return 1;}assert!(miller_rabin_test(p));let mut factor=factorize(p-1);factor.sort_unstable();factor.dedup();for g in 1..{let mg=ArbitraryMontgomeryModint::new(g,p);if factor.iter().all(|&f|mg.pow((p-1)/f)!=mg.one()){return g%p;}}unreachable!()}}
        pub mod __numeric_0_1_0 {use crate::__cargo_equip::preludes::__numeric_0_1_0::*;pub mod float{use crate::__cargo_equip::preludes::__numeric_0_1_0::*;use super::Numeric;use std::ops::Neg;macro_rules!impl_numeric_trait_for_float{($($t:tt)*)=>{$(impl Numeric for$t{fn max_value()->Self{std::$t::MAX}fn min_value()->Self{std::$t::MIN}})*};}impl_numeric_trait_for_float!(f32 f64);pub trait Float:Numeric+Neg<Output=Self>{fn abs(self)->Self;fn acos(self)->Self;fn asin(self)->Self;fn atan(self)->Self;fn atan2(self,other:Self)->Self;fn cbrt(self)->Self;fn ceil(self)->Self;fn cos(self)->Self;fn floor(self)->Self;fn hypot(self,other:Self)->Self;fn is_infinite(self)->bool;fn is_nan(self)->bool;fn max(self,other:Self)->Self;fn min(self,other:Self)->Self;fn mul_add(self,a:Self,b:Self)->Self;fn powf(self,n:Self)->Self;fn powi(self,n:i32)->Self;fn round(self)->Self;fn signum(self)->Self;fn sin(self)->Self;fn sqrt(self)->Self;fn tan(self)->Self;fn to_radians(self)->Self;fn pi()->Self;}macro_rules!impl_float_trait{($($t:tt)*)=>{$(impl Float for$t{fn abs(self)->Self{self.abs()}fn acos(self)->Self{self.acos()}fn asin(self)->Self{self.asin()}fn atan(self)->Self{self.atan()}fn atan2(self,other:Self)->Self{self.atan2(other)}fn cbrt(self)->Self{self.cbrt()}fn ceil(self)->Self{self.ceil()}fn cos(self)->Self{self.cos()}fn floor(self)->Self{self.floor()}fn hypot(self,other:Self)->Self{self.hypot(other)}fn is_infinite(self)->bool{self.is_infinite()}fn is_nan(self)->bool{self.is_nan()}fn max(self,other:Self)->Self{self.max(other)}fn min(self,other:Self)->Self{self.min(other)}fn mul_add(self,a:Self,b:Self)->Self{self.mul_add(a,b)}fn powf(self,n:Self)->Self{self.powf(n)}fn powi(self,n:i32)->Self{self.powi(n)}fn round(self)->Self{self.round()}fn signum(self)->Self{self.signum()}fn sin(self)->Self{self.sin()}fn sqrt(self)->Self{self.sqrt()}fn tan(self)->Self{self.tan()}fn to_radians(self)->Self{self.to_radians()}fn pi()->Self{std::$t::consts::PI}})*};}impl_float_trait!(f32 f64);}pub mod integer{use crate::__cargo_equip::preludes::__numeric_0_1_0::*;use super::Numeric;use std::ops::{BitAnd,BitAndAssign,BitOr,BitOrAssign,BitXor,BitXorAssign,Rem,RemAssign,Shl,ShlAssign,Shr,ShrAssign};macro_rules!impl_numeric_trait_for_integer{($($t:tt)*)=>{$(impl Numeric for$t{fn max_value()->Self{std::$t::MAX}fn min_value()->Self{std::$t::MIN}})*};}impl_numeric_trait_for_integer!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);pub trait Integer:Numeric+Rem<Self,Output=Self>+RemAssign+Shl<i32,Output=Self>+Shl<i64,Output=Self>+Shl<u32,Output=Self>+Shl<u64,Output=Self>+Shl<usize,Output=Self>+Shr<i32,Output=Self>+Shr<i64,Output=Self>+Shr<u32,Output=Self>+Shr<u64,Output=Self>+Shr<usize,Output=Self>+ShlAssign<i32>+ShlAssign<i64>+ShlAssign<u32>+ShlAssign<u64>+ShlAssign<usize>+ShrAssign<i32>+ShrAssign<i64>+ShrAssign<u32>+ShrAssign<u64>+ShrAssign<usize>+BitAnd<Self,Output=Self>+BitOr<Self,Output=Self>+BitXor<Self,Output=Self>+BitAndAssign+BitOrAssign+BitXorAssign+std::hash::Hash+Eq+Ord{fn abs_diff(self,other:Self)->Self;fn count_ones(self)->u32;fn count_zeros(self)->u32;fn div_euclid(self,rhs:Self)->Self;fn leading_ones(self)->u32;fn leading_zeros(self)->u32;fn rem_euclid(self,rhs:Self)->Self;fn reverse_bits(self)->Self;fn rotate_left(self,n:u32)->Self;fn rotate_right(self,n:u32)->Self;fn trailing_ones(self)->u32;fn trailing_zeros(self)->u32;fn overflowing_add(self,rhs:Self)->(Self,bool);fn overflowing_mul(self,rhs:Self)->(Self,bool);fn overflowing_neg(self)->(Self,bool);fn overflowing_sub(self,rhs:Self)->(Self,bool);fn saturating_add(self,rhs:Self)->Self;fn saturating_mul(self,rhs:Self)->Self;fn saturating_sub(self,rhs:Self)->Self;fn wrapping_add(self,rhs:Self)->Self;fn wrapping_mul(self,rhs:Self)->Self;fn wrapping_neg(self)->Self;fn wrapping_sub(self,rhs:Self)->Self;}macro_rules!impl_integer_trait{($($t:ty)*)=>{$(impl Integer for$t{fn abs_diff(self,other:Self)->Self{std::cmp::max(self,other)-std::cmp::min(self,other)}fn count_ones(self)->u32{self.count_ones()}fn count_zeros(self)->u32{self.count_zeros()}fn div_euclid(self,rhs:Self)->Self{self.div_euclid(rhs)}fn leading_ones(self)->u32{(!self).leading_zeros()}fn leading_zeros(self)->u32{self.leading_zeros()}fn rem_euclid(self,rhs:Self)->Self{self.rem_euclid(rhs)}fn reverse_bits(self)->Self{self.reverse_bits()}fn rotate_left(self,n:u32)->Self{self.rotate_left(n)}fn rotate_right(self,n:u32)->Self{self.rotate_right(n)}fn trailing_ones(self)->u32{(!self).trailing_zeros()}fn trailing_zeros(self)->u32{self.trailing_zeros()}fn overflowing_add(self,rhs:Self)->(Self,bool){self.overflowing_add(rhs)}fn overflowing_mul(self,rhs:Self)->(Self,bool){self.overflowing_mul(rhs)}fn overflowing_neg(self)->(Self,bool){self.overflowing_neg()}fn overflowing_sub(self,rhs:Self)->(Self,bool){self.overflowing_sub(rhs)}fn saturating_add(self,rhs:Self)->Self{self.saturating_add(rhs)}fn saturating_mul(self,rhs:Self)->Self{self.saturating_mul(rhs)}fn saturating_sub(self,rhs:Self)->Self{self.saturating_sub(rhs)}fn wrapping_add(self,rhs:Self)->Self{self.wrapping_add(rhs)}fn wrapping_mul(self,rhs:Self)->Self{self.wrapping_mul(rhs)}fn wrapping_neg(self)->Self{self.wrapping_neg()}fn wrapping_sub(self,rhs:Self)->Self{self.wrapping_sub(rhs)}})*};}impl_integer_trait!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);}pub mod signed{use crate::__cargo_equip::preludes::__numeric_0_1_0::*;use std::ops::Neg;pub trait Signed:Neg<Output=Self>+std::marker::Sized{fn is_negative(&self)->bool;fn is_positive(&self)->bool;}macro_rules!impl_integer_trait{($($t:ty)*)=>{$(impl Signed for$t{fn is_negative(&self)->bool{Self::is_negative(*self)}fn is_positive(&self)->bool{Self::is_positive(*self)}})*};}impl_integer_trait!(i8 i16 i32 i64 i128 isize);}pub use float::Float;pub use integer::Integer;pub use signed::Signed;pub use zero_one::{One,Zero};use std::ops::{Add,AddAssign,Div,DivAssign,Mul,MulAssign,Sub,SubAssign};#[derive(Debug)]pub struct Error(pub&'static str);impl std::fmt::Display for Error{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"{}",self.0)}}impl std::error::Error for Error{}pub trait UnorderedNumeric:Add<Self,Output=Self>+Sub<Self,Output=Self>+Mul<Self,Output=Self>+Div<Self,Output=Self>+AddAssign+SubAssign+MulAssign+DivAssign+std::fmt::Debug+std::fmt::Display+Clone+Copy+PartialEq+Default+Zero+One{}pub trait Numeric:Add<Self,Output=Self>+Sub<Self,Output=Self>+Mul<Self,Output=Self>+Div<Self,Output=Self>+AddAssign+SubAssign+MulAssign+DivAssign+std::fmt::Debug+std::fmt::Display+Clone+Copy+PartialEq+PartialOrd+Default+Zero+One{fn max_value()->Self;fn min_value()->Self;}pub trait IntoFloat:Numeric{fn as_f64(self)->f64;fn as_f32(self)->f32;}impl IntoFloat for i64{fn as_f64(self)->f64{self as f64}fn as_f32(self)->f32{self as f32}}}
        pub mod __simple_rand_0_1_0 {use std::collections::hash_map::RandomState;use std::hash::{BuildHasher,Hasher};pub fn gen_seed()->u64{let hasher=RandomState::new().build_hasher();hasher.finish()}pub fn xor_shift32(seed:u64)->impl Iterator<Item=u32>{let mut random=seed as u32;std::iter::repeat_with(move||{random^=random<<13;random^=random>>17;random^=random<<5;random})}pub fn xor_shift(seed:u64)->impl Iterator<Item=u64>{let mut random=seed;std::iter::repeat_with(move||{random^=random<<13;random^=random>>7;random^=random<<17;random})}pub fn xor_shift128(seed:u64)->impl Iterator<Item=u128>{let mut x64=xor_shift(seed);std::iter::repeat_with(move||(x64.next().unwrap()as u128)<<64|x64.next().unwrap()as u128)}}
        pub mod __zero_one_0_1_0 {mod one{pub trait One{fn one()->Self;}macro_rules!impl_one_integer{($($t:ty)*)=>{$(impl One for$t{fn one()->$t{1}})*};}impl_one_integer!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);macro_rules!impl_one_float{($($t:ty)*)=>{$(impl One for$t{fn one()->$t{1.0}})*};}impl_one_float!(f32 f64);}mod zero{pub trait Zero{fn zero()->Self;}macro_rules!impl_zero_integer{($($t:ty)*)=>{$(impl Zero for$t{fn zero()->$t{0}})*};}impl_zero_integer!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);macro_rules!impl_zero_float{($($t:ty)*)=>{$(impl Zero for$t{fn zero()->$t{0.0}})*};}impl_zero_float!(f32 f64);}pub use one::*;pub use zero::*;}
    }

    pub(crate) mod macros {
        pub mod __arbitrary_montgomery_modint_0_1_0 {}
        pub mod math {}
        pub mod __numeric_0_1_0 {}
        pub mod __simple_rand_0_1_0 {}
        pub mod __zero_one_0_1_0 {}
    }

    pub(crate) mod prelude {pub use crate::__cargo_equip::crates::*;}

    mod preludes {
        pub mod __arbitrary_montgomery_modint_0_1_0 {}
        pub mod math {pub(in crate::__cargo_equip)use crate::__cargo_equip::crates::{__arbitrary_montgomery_modint_0_1_0 as arbitrary_montgomery_modint,__numeric_0_1_0 as numeric,__simple_rand_0_1_0 as simple_rand};}
        pub mod __numeric_0_1_0 {pub(in crate::__cargo_equip)use crate::__cargo_equip::crates::__zero_one_0_1_0 as zero_one;}
        pub mod __simple_rand_0_1_0 {}
        pub mod __zero_one_0_1_0 {}
    }
}
