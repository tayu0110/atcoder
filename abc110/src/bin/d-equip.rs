pub use __ce::prl::*;use mat::{dvs,fact};use mi::{comb,Mod1000000007,Mi};use proconio::*;type M=Mi<Mod1000000007>;fn main(){input!{n:usize,m:usize}if m==1{println!("1");return;}let mut f=fact(m as u64);f.sort();let mut d=dvs(m as u64);d.sort();let mut dp=vec![vec![M::zero(); d.len()]; f.len()+1];dp[0][d.len()-1]=M::one();for i in 0..f.len(){for j in 1..d.len(){if dp[i][j]==M::zero(){continue;}let mut nw=j;for k in 1..=j{if d[j]%d[k]!=0{continue;}let nt=d[j]/d[k];while d[nw]!=nt{nw-=1;}dp[i+1][nw]=dp[i+1][nw]+dp[i][j];}}}let com=comb::<Mod1000000007>(n as u32+10);let mut r=M::zero();for i in 1..=f.len(){if i>n{break;}r+=com(n,i)*dp[i][0];}println!("{}",r);}#[allow(unused)]mod __ce{pub(crate) mod crates{pub mod mat{use crate::__ce::preludes::mat::*;use mi::DMM;use numeric::Integer;#[inline]pub fn gcd<T:Integer>(mut x:T,mut y:T)->T{while y!=T::zero(){let(nx,ny)=(y,x%y);x=nx;y=ny;}x}#[inline]pub fn lcm<T:Integer>(x:T,y:T)->T{x/gcd(x,y)*y}#[inline]pub fn ext_gcd<T:Integer>(a:T,b:T)->(T,T,T){let(mut s,mut t)=(a,b);let(mut sx,mut tx)=(T::one(),T::zero());let(mut sy,mut ty)=(T::zero(),T::one());while s%t!=T::zero(){let d=s/t;let u=s%t;let ux=sx-tx*d;let uy=sy-ty*d;s=t;sx=tx;sy=ty;t=u;tx=ux;ty=uy;}(t,tx,ty)}#[inline]pub fn mpow(a:i64,mut n:i64,p:i64)->i64{let mut r=1;let mut pow=a;while n!=0{if n&1!=0{r=(r as i128*pow as i128%p as i128) as i64;}pow=(pow as i128*pow as i128%p as i128) as i64;n>>=1;}r}#[inline]pub fn mod_log(a:i64,b:i64,p:i64)->Option<i64>{mlwlbc(a,b,p,0)}pub fn mlwlbc(a:i64,b:i64,p:i64,lower:i64)->Option<i64>{let(a,b)=(a.rem_euclid(p),b.rem_euclid(p));if p==1{return Some(lower);}if b==1 && lower<=0{return Some(0);}let(g,iv,_)=ext_gcd(a,p);if g!=1{if b%g!=0{return None;}let(na,nb,np)=(a/g,b/g,p/g);let(_,iv,_)=ext_gcd(na,np);let iv=iv.rem_euclid(np);if let Some(r)=mod_log(a,nb*iv,np){return Some(r+1);}else{return None;}}let m=(p as f64).sqrt().ceil() as i64;assert!(m*m>=p);let mut nw=1;let mut mp=std::collections::HashMap::new();for j in 0..m{mp.entry(nw).or_insert(vec![]).push(j);nw=(nw as i128*a as i128%p as i128) as i64;}let iv=mpow(iv.rem_euclid(p),m,p);debug_assert_eq!((nw as i128*iv as i128).rem_euclid(p as i128),1);let mut nw=1;for i in 0..=m{let r=(b as i128*nw as i128%p as i128) as i64;if let Some(v)=mp.get(&r){for j in v{if i*m+j<lower{continue;}return Some(i*m+j);}}nw=(nw as i128*iv as i128%p as i128) as i64;}None}pub fn bsgs(a:i64,b:i64,p:i64,f:impl Fn(i64,i64)->i64,f_inv:impl Fn(i64,i64)->i64)->Option<i64>{let m=(p as f64).sqrt().ceil() as i64;assert!(m*m>=p);let mut mp=std::collections::HashMap::new();for j in 0..=m{let nw=f(a,j);if !mp.contains_key(&nw){mp.insert(nw,j);}}let mut nw=f_inv(b,0);for i in 0..=m{if let Some(j)=mp.get(&nw){return Some(i*m+j);}nw=f_inv(nw,m);}None}#[inline]pub fn crt(mut a:i64,mut m1:i64,mut b:i64,mut m2:i64)->Option<(i64,i64)>{if m1<m2{std::mem::swap(&mut a,&mut b);std::mem::swap(&mut m1,&mut m2);}let(a,b)=(a.rem_euclid(m1),b.rem_euclid(m2));if m1%m2==0{return if a%m2!=b{None}else{Some((a,m1))};}let(d,k,_)=ext_gcd(m1,m2);let u1=m2/d;if a%d!=b%d{return None;}let x=(b-a)/d%u1*k%u1;let m=m1*u1;let r=(a+x*m1).rem_euclid(m);Some((r,m))}#[inline]pub fn garner(a:&[i64],p:&[i64],mdl:i64)->(i64,i64){assert_eq!(a.len(),p.len());let mut pd=vec![1; p.len()+1];let mut r=vec![0; p.len()+1];for (i,(&a,&m)) in a.iter().zip(p.iter()).enumerate(){let a=a%m;let(_,iv,_)=ext_gcd(pd[i],m);let t=((a-r[i])*iv).rem_euclid(m);for (i,&p) in p.iter().enumerate().skip(i+1){r[i]=(r[i]+(t*pd[i]))%p;pd[i]=(pd[i]*m)%p;}r[p.len()]=(r[p.len()]+(t*pd[p.len()]))%mdl;pd[p.len()]=(pd[p.len()]*m)%mdl;}(r[p.len()],pd[p.len()])}#[inline]pub fn garnerp(a:&[i64],p:&[i64],mdl:i64)->Option<(i64,i64)>{let mut p=p.to_vec();for i in 0..a.len(){for j in 0..i{let mut g=gcd(p[i],p[j]);if (a[i]-a[j])%g!=0{return None;}p[i]/=g;p[j]/=g;let mut gi=gcd(p[i],g);let mut gj=g/gi;g=gcd(gi,gj);gi*=g;gj/=g;while g!=1{g=gcd(gi,gj);gi*=g;gj/=g;}p[i]*=gi;p[j]*=gj;}}Some(garner(a,&p,mdl))}pub fn mrt(p:u64)->bool{if p==1||p&1==0{return p==2;}let s=(p-1).trailing_zeros();let t=(p-1)>>s;let mz=DMM::raw(0,p);let mo=mz.one();let mno=mz-mo;vec![2,325,9375,28178,450775,9780504,1795265022].iter().map(|&a|a%p).filter(|&a|a!=0).all(|a|{let a=DMM::fsmuc(a,mz);let at=a.pow(t);if at==mo||at==mno{return true;}(1..s).scan(at,|at,_|{*at*=*at;Some(*at)}).any(|at|at==mno)})}pub fn dvs(n:u64)->Vec<u64>{let mut f=fact(n);f.sort();let mut t=vec![];for f in f{match t.last_mut(){Some((c,cnt)) if *c==f=> *cnt+=1,_=> t.push((f,1))}}let mut r=vec![1];for (c,cnt) in t{let mut nw=1;let len=r.len();for _ in 0..cnt{nw*=c;for i in 0..len{let new=r[i]*nw;r.push(new);}}}r}pub fn fact(mut n:u64)->Vec<u64>{if n==1{return vec![];}let mut r=vec![2u64; n.trailing_zeros() as usize];n>>=n.trailing_zeros();while let Some(g)=prh(n){while n%g==0{r.push(g);n/=g;}}if n>1{r.push(n);}r}fn prh(n:u64)->Option<u64>{if n<=1{return None;}if n&1==0{return Some(2);}if mrt(n){return Some(n);}let m=(n as f64).powf(0.125).round() as i64+1;let mz=DMM::raw(0,n);let mo=mz.one();let mut g=1;for c in(1..n).map(|c|DMM::fsmuc(c,mz)){let mut y=mz;let mut q=mo;'base:for r in (0..).map(|i|1<<i){let x=y;(r..=(3*r)>>2).for_each(|_|y=y*y+c);for k in (((3*r)>>2)..r).step_by(m as usize){let ys=y;(0..std::cmp::min(m,r-k)).for_each(|_|{y=y*y+c;q*=x-y;});g=gcd(q.v() as i64,n as i64);if g!=1{if g==n as i64{y=ys*ys+c;while gcd((x-y).v() as i64,n as i64)==1{y=y*y+c;}g=gcd((x-y).v() as i64,n as i64);}break 'base;}}}if g!=n as i64{break;}}prh(g as u64)}}pub mod mi{use crate::__ce::preludes::mi::*;mod dm{use crate::__ce::preludes::mi::*;use std::ops::{Add,AddAssign,Div,DivAssign,Mul,MulAssign,Sub,SubAssign};#[derive(Clone,Copy,PartialEq,Eq)]pub struct DM{v:u64,mdl:u64}impl DM{#[inline]pub const fn new(v:u64,mdl:u64)->Self{Self::raw(v%mdl,mdl)}#[inline]pub const fn raw(v:u64,mdl:u64)->Self{Self{v,mdl}}#[inline]pub const fn one(mdl:u64)->Self{Self::new(1,mdl)}#[inline]pub const fn zero(mdl:u64)->Self{Self::raw(0,mdl)}#[inline]pub const fn add_raw(&self,rs:u64)->Self{let(t,fa)=self.v.overflowing_add(rs);let(u,fs)=t.overflowing_sub(self.mdl);let f=fa as u64 |!fs as u64;Self{v:f*u+(1-f)*t,mdl:self.mdl}}#[inline]pub const fn sub_raw(&self,rs:u64)->Self{let(v,f)=self.v.overflowing_sub(rs);Self{v:v.wrapping_add(self.mdl*f as u64),mdl:self.mdl}}#[inline]pub const fn mul_raw(&self,rs:u64)->Self{Self{v:(self.v as u128*rs as u128%self.mdl as u128) as u64,mdl:self.mdl}}#[inline]pub fn pow(&self,mut e:u64)->Self{let(mut v,mut r)=(self.v as u64,1);while e>0{if e&1==1{r=(r as u128*v as u128%self.mdl as u128) as u64;}v=(v as u128*v as u128%self.mdl as u128) as u64;e>>=1;}Self{v:r,mdl:self.mdl}}#[inline]pub fn iv(&self)->Self{let(mut s,mut ys)=(self.mdl,0u64);let(mut t,mut yt)=(self.v,1u64);while s%t!=0{let tmp=s/t;let u=s-t*tmp;let(v,f)=yt.overflowing_mul(tmp);let yu=if f||v>=self.mdl{ys.wrapping_add(yt.wrapping_neg()*tmp)}else{ys.wrapping_sub(v)};s=t;ys=yt;t=u;yt=yu;}if yt>self.mdl{yt=yt.wrapping_add(self.mdl);}assert_eq!(t,1);assert_eq!(self.v as u128*yt as u128%self.mdl as u128,1);Self{v:yt,mdl:self.mdl}}}impl std::fmt::Debug for DM{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"{}",self.v)}}impl std::fmt::Display for DM{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"{}",self.v)}}impl Add for DM{type Output=Self;fn add(self,rs:Self)->Self::Output{self.add_raw(rs.v)}}impl AddAssign for DM{fn add_assign(&mut self,rs:Self){*self=*self+rs;}}impl Sub for DM{type Output=Self;fn sub(self,rs:Self)->Self::Output{self.sub_raw(rs.v)}}impl SubAssign for DM{fn sub_assign(&mut self,rs:Self){*self=*self-rs;}}impl Mul for DM{type Output=Self;fn mul(self,rs:Self)->Self::Output{self.mul_raw(rs.v)}}impl MulAssign for DM{fn mul_assign(&mut self,rs:Self){*self=*self*rs;}}impl Div for DM{type Output=Self;fn div(self,rs:Self)->Self::Output{debug_assert!(rs.v!=0);self*rs.iv()}}impl DivAssign for DM{fn div_assign(&mut self,rs:Self){debug_assert!(rs.v!=0);*self*=rs.iv()}}}mod dmm{use crate::__ce::preludes::mi::*;use std::ops::{Add,AddAssign,Div,DivAssign,Mul,MulAssign,Sub,SubAssign};const fn mred(v:u64,mdl:u64,miv:u64)->u64{let(t,f)=(((v.wrapping_mul(miv) as u128).wrapping_mul(mdl as u128)>>64) as u64).overflowing_neg();t.wrapping_add(mdl*f as u64)}const fn mmul(lhs:u64,rs:u64,mdl:u64,miv:u64)->u64{let a=lhs as u128*rs as u128;let(t,f)=((a>>64) as u64).overflowing_sub((((a as u64).wrapping_mul(miv) as u128).wrapping_mul(mdl as u128)>>64) as u64);t.wrapping_add(mdl*f as u64)}#[derive(Clone,Copy,PartialEq,Eq)]pub struct DMM{pub v:u64,mdl:u64,miv:u64,r:u64,r2:u64}impl DMM{#[inline]pub const fn new(v:u64,mdl:u64)->Self{Self::raw(v%mdl,mdl)}pub const fn raw(v:u64,mdl:u64)->Self{let r=((1u128<<64)%mdl as u128) as u64;let r2=((mdl as u128).wrapping_neg()%mdl as u128) as u64;let miv={let iv=mdl.wrapping_mul(2u64.wrapping_sub(mdl.wrapping_mul(mdl)));let iv=iv.wrapping_mul(2u64.wrapping_sub(mdl.wrapping_mul(iv)));let iv=iv.wrapping_mul(2u64.wrapping_sub(mdl.wrapping_mul(iv)));let iv=iv.wrapping_mul(2u64.wrapping_sub(mdl.wrapping_mul(iv)));iv.wrapping_mul(2u64.wrapping_sub(mdl.wrapping_mul(iv)))};let v=mmul(v,r2,mdl,miv);Self{v,mdl,miv,r,r2}}#[inline]const fn frpu(v:u64,mdl:u64,miv:u64,r:u64,r2:u64)->Self{Self{v,mdl,miv,r,r2}}#[inline]pub const fn from_same_mod(v:u64,from:Self)->Self{Self::fsmuc(v%from.mdl,from)}#[inline]pub const fn fsmuc(v:u64,from:Self)->Self{let v=mmul(v,from.r2,from.mdl,from.miv);Self::frpu(v,from.mdl,from.miv,from.r,from.r2)}#[inline]pub const fn v(&self)->u64{mred(self.v,self.mdl,self.miv)}#[inline]pub const fn val_mont_expr(&self)->u64{self.v}#[inline]pub const fn one(&self)->Self{Self{v:self.r,mdl:self.mdl,miv:self.miv,r:self.r,r2:self.r2}}#[inline]pub const fn zero(&self)->Self{Self{v:0,mdl:self.mdl,miv:self.miv,r:self.r,r2:self.r2}}pub fn pow(&self,mut n:u64)->Self{let mut v=self.v;let mut r=self.r;while n!=0{if n&1!=0{r=mmul(r,v,self.mdl,self.miv);}v=mmul(v,v,self.mdl,self.miv);n>>=1;}Self{v:r,mdl:self.mdl,miv:self.miv,r:self.r,r2:self.r2}}#[inline]pub fn iv(&self)->Self{self.pow(self.mdl-2)}}impl Add for DMM{type Output=Self;fn add(self,rs:Self)->Self::Output{let(t,fa)=self.v.overflowing_add(rs.v);let(u,fs)=t.overflowing_sub(self.mdl);Self{v:if fa||!fs{u}else{t},mdl:self.mdl,miv:self.miv,r:self.r,r2:self.r2}}}impl Sub for DMM{type Output=Self;fn sub(self,rs:Self)->Self::Output{let(v,f)=self.v.overflowing_sub(rs.v);Self{v:if f{v.wrapping_add(self.mdl)}else{v},mdl:self.mdl,miv:self.miv,r:self.r,r2:self.r2}}}impl Mul for DMM{type Output=Self;fn mul(self,rs:Self)->Self::Output{Self{v:mmul(self.v,rs.v,self.mdl,self.miv),mdl:self.mdl,miv:self.miv,r:self.r,r2:self.r2}}}impl Div for DMM{type Output=Self;fn div(self,rs:Self)->Self::Output{self*rs.iv()}}impl AddAssign for DMM{fn add_assign(&mut self,rs:Self){*self=*self+rs;}}impl SubAssign for DMM{fn sub_assign(&mut self,rs:Self){*self=*self-rs;}}impl MulAssign for DMM{fn mul_assign(&mut self,rs:Self){*self=*self*rs;}}impl DivAssign for DMM{fn div_assign(&mut self,rs:Self){*self=*self/rs;}}impl std::fmt::Debug for DMM{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"{}",self.v())}}impl std::fmt::Display for DMM{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"{}",self.v())}}}mod mdl{use crate::__ce::preludes::mi::*;use std::{fmt::Debug,marker};pub trait Mdl:Clone+marker::Copy+PartialEq+Eq+Debug{const M:u32;const MOD_INV:u32={let iv=Self::M.wrapping_mul(2u32.wrapping_sub(Self::M.wrapping_mul(Self::M)));let iv=iv.wrapping_mul(2u32.wrapping_sub(Self::M.wrapping_mul(iv)));let iv=iv.wrapping_mul(2u32.wrapping_sub(Self::M.wrapping_mul(iv)));let iv=iv.wrapping_mul(2u32.wrapping_sub(Self::M.wrapping_mul(iv)));iv.wrapping_mul(2u32.wrapping_sub(Self::M.wrapping_mul(iv)))};const R:u32=((1u64<<32)%Self::M as u64) as u32;const R2:u32=((Self::M as u64).wrapping_neg()%Self::M as u64) as u32;const PR:u32;}macro_rules! impl_modulo{($({$name:ident,$mdl:literal,$prim_root:literal},)*)=>{$(#[derive(Debug,Clone,marker::Copy,PartialEq,Eq)]pub enum $name{}impl Mdl for $name{const M:u32=$mdl;const PR:u32=$prim_root;})*};}impl_modulo!({Mod167772161,167772161,3},{Mod377487361,377487361,7},{Mod469762049,469762049,3},{Mod595591169,595591169,3},{Mod645922817,645922817,3},{Mod754974721,754974721,11},{Mod880803841,880803841,26},{Mod897581057,897581057,3},{Mod998244353,998244353,3},{Mod1000000007,1000000007,5},{Mod1107296257,1107296257,10},{Mod1224736769,1224736769,3},{Mod1300234241,1300234241,3},{Mod1484783617,1484783617,5},{Mod1711276033,1711276033,29},{Mod1811939329,1811939329,13},{Mod2013265921,2013265921,31},{Mod2088763393,2088763393,5},{Mod2113929217,2113929217,5},{Mod2130706433,2130706433,3},{Mod2281701377,2281701377,3},{Mod2483027969,2483027969,3},{Mod2533359617,2533359617,3},{Mod2634022913,2634022913,3},{Mod2717908993,2717908993,5},{Mod2868903937,2868903937,35},{Mod2885681153,2885681153,3},{Mod3221225473,3221225473,5},{Mod3238002689,3238002689,3},{Mod3489660929,3489660929,3},{Mod3892314113,3892314113,3},{Mod3942645761,3942645761,3},{Mod4076863489,4076863489,7},{Mod4194304001,4194304001,3},);}mod mm{use super::mdl::*;use crate::__ce::preludes::mi::*;use numeric::{One,Zero};use std::convert::From;use std::marker::PhantomData;use std::num::ParseIntError;use std::ops::{Add,AddAssign,Div,DivAssign,Mul,MulAssign,Neg,Sub,SubAssign};use std::str::FromStr;#[derive(Clone,Copy,PartialEq,Eq)]pub struct MM<M:Mdl>{pub v:u32,_p:PhantomData<fn()->M>}impl<M:Mdl> MM<M>{fn mred(v:u32)->u32{let(t,f)=(((v.wrapping_mul(M::MOD_INV) as u64).wrapping_mul(M::M as u64)>>32) as u32).overflowing_neg();t.wrapping_add(M::M*f as u32)}fn mmul(lhs:u32,rs:u32)->u32{let a=lhs as u64*rs as u64;let(t,f)=((a>>32) as u32).overflowing_sub((((a as u32).wrapping_mul(M::MOD_INV) as u64).wrapping_mul(M::M as u64)>>32) as u32);t.wrapping_add(M::M*f as u32)}#[inline]pub fn new(v:u32)->Self{Self::raw(v.rem_euclid(M::M))}pub fn raw(v:u32)->Self{let v=Self::mmul(v,M::R2);Self{v,_p:PhantomData}}#[inline]pub fn fme(v:u32)->Self{Self{v,_p:PhantomData}}#[inline]pub fn v(&self)->u32{Self::mred(self.v)}#[inline]pub fn val_mont_expr(&self)->u32{self.v}#[inline]pub fn one()->Self{Self{v:M::R,_p:PhantomData}}#[inline]pub fn zero()->Self{Self{v:0,_p:PhantomData}}pub fn pow(&self,mut n:u64)->Self{let mut v=self.v;let mut r=M::R;while n!=0{if n&1!=0{r=Self::mmul(r,v);}v=Self::mmul(v,v);n>>=1;}Self{v:r,_p:PhantomData}}#[inline]pub fn nth_root(n:u32)->Self{debug_assert!(n==1<<n.trailing_zeros());MM::<M>::new(M::PR).pow(M::M as u64-1+(M::M as u64-1)/n as u64)}#[inline]pub fn iv(&self)->Self{self.pow(M::M as u64-2)}}impl<M:Mdl> One for MM<M>{#[inline]fn one()->Self{Self::one()}}impl<M:Mdl> Zero for MM<M>{#[inline]fn zero()->Self{Self::zero()}}impl<M:Mdl> Add for MM<M>{type Output=Self;fn add(self,rs:Self)->Self::Output{let(t,fa)=self.v.overflowing_add(rs.v);let(u,fs)=t.overflowing_sub(M::M);Self{v:if fa||!fs{u}else{t},_p:PhantomData}}}impl<M:Mdl> Sub for MM<M>{type Output=Self;fn sub(self,rs:Self)->Self::Output{let(v,f)=self.v.overflowing_sub(rs.v);Self{v:if f{v.wrapping_add(M::M)}else{v},_p:PhantomData}}}impl<M:Mdl> Mul for MM<M>{type Output=Self;fn mul(self,rs:Self)->Self::Output{Self{v:Self::mmul(self.v,rs.v),_p:PhantomData}}}impl<M:Mdl> Div for MM<M>{type Output=Self;fn div(self,rs:Self)->Self::Output{self*rs.iv()}}impl<M:Mdl> Neg for MM<M>{type Output=Self;fn neg(self)->Self::Output{if self.v==0{self}else{Self{v:M::M-self.v,_p:PhantomData}}}}impl<M:Mdl> AddAssign for MM<M>{fn add_assign(&mut self,rs:Self){*self=*self+rs;}}impl<M:Mdl> SubAssign for MM<M>{fn sub_assign(&mut self,rs:Self){*self=*self-rs;}}impl<M:Mdl> MulAssign for MM<M>{fn mul_assign(&mut self,rs:Self){*self=*self*rs;}}impl<M:Mdl> DivAssign for MM<M>{fn div_assign(&mut self,rs:Self){*self=*self/rs;}}impl<M:Mdl> std::fmt::Debug for MM<M>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"{}",self.v())}}impl<M:Mdl> std::fmt::Display for MM<M>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"{}",self.v())}}impl<M:Mdl> From<u32> for MM<M>{fn from(value:u32)->Self{Self::new(value)}}impl<M:Mdl> From<u64> for MM<M>{fn from(value:u64)->Self{Self::raw(value.rem_euclid(M::M as u64) as u32)}}impl<M:Mdl> From<i32> for MM<M>{fn from(value:i32)->Self{Self::raw(value.rem_euclid(M::M as i32) as u32)}}impl<M:Mdl> From<i64> for MM<M>{fn from(value:i64)->Self{Self::raw(value.rem_euclid(M::M as i64) as u32)}}impl<M:Mdl> FromStr for MM<M>{type Err=ParseIntError;fn from_str(s:&str)->Result<Self,Self::Err>{let ng=s.starts_with("-");let v=if ng{s[1..].bytes().fold(0u64,|s,v|s*10+(v-b'0') as u64)}else{s.bytes().fold(0u64,|s,v|s*10+(v-b'0') as u64)};if !ng && v<M::M as u64{Ok(Self::raw(v as u32))}else if ng{Ok(Self::from(-(v as i64)))}else{Ok(Self::from(v))}}}}pub use dm::*;pub use dmm::*;pub use mdl::*;pub use mm::*;use numeric::{One,Zero};use std::marker::{self,PhantomData};use std::ops::{Add,AddAssign,Div,DivAssign,Mul,MulAssign,Sub,SubAssign};#[derive(Clone,Copy,PartialEq,Eq)]pub struct Mi<M:Mdl>{v:u32,_p:PhantomData<fn()->M>}impl<M:Mdl> One for Mi<M>{#[inline]fn one()->Self{Self::one()}}impl<M:Mdl> Zero for Mi<M>{#[inline]fn zero()->Self{Self::zero()}}impl<M:Mdl> Mi<M>{#[inline]pub fn new(v:u64)->Self{Mi{v:(v%M::M as u64) as u32,_p:PhantomData}}pub fn new_signed(v:i64)->Self{Mi{v:v.rem_euclid(M::M as i64) as u32,_p:PhantomData}}#[inline]pub fn raw(v:u32)->Self{debug_assert!(v<M::M);Mi{v,_p:marker::PhantomData}}#[inline]pub fn zero()->Self{Mi{v:0,_p:marker::PhantomData}}#[inline]pub fn one()->Self{Mi{v:1,_p:marker::PhantomData}}#[inline]pub fn mdl()->u32{M::M}#[inline]pub fn v(&self)->u32{self.v}pub fn pow(&self,mut e:u32)->Self{let(mut v,mut r)=(self.v as u64,1);while e>0{if e&1==1{r=(r*v)%M::M as u64;}v=(v*v)%M::M as u64;e>>=1;}Self{v:r as u32,_p:PhantomData}}#[inline]pub fn iv(&self)->Self{self.pow(M::M-2)}#[inline]pub fn nth_root(n:u32)->Self{debug_assert!(n==1<<n.trailing_zeros());Mi::raw(M::PR).pow(M::M-1+(M::M-1)/n)}#[inline]pub fn add_raw(&self,rs:u32)->Self{debug_assert!(rs<M::M);let r=self.v+rs;Mi::raw(if r>=M::M{r-M::M}else{r})}#[inline]pub fn sub_raw(&self,rs:u32)->Self{debug_assert!(rs<M::M);let(r,f)=self.v.overflowing_sub(rs);Mi::raw(if f{r.wrapping_add(M::M)}else{r})}#[inline]pub fn mul_raw(&self,rs:u32)->Self{debug_assert!(rs<M::M);Mi::new(self.v as u64*rs as u64)}#[inline]pub fn div_raw(&self,rs:u32)->Self{debug_assert!(rs<M::M);*self/Mi::raw(rs)}}impl<M:Mdl> Default for Mi<M>{fn default()->Self{Mi::zero()}}impl<M:Mdl> std::fmt::Debug for Mi<M>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"{}",self.v)}}impl<M:Mdl> std::fmt::Display for Mi<M>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"{}",self.v)}}impl<M:Mdl> Add for Mi<M>{type Output=Self;fn add(self,rs:Self)->Self::Output{self.add_raw(rs.v)}}impl<M:Mdl> AddAssign for Mi<M>{fn add_assign(&mut self,rs:Self){*self=*self+rs;}}impl<M:Mdl> Sub for Mi<M>{type Output=Self;fn sub(self,rs:Self)->Self::Output{self.sub_raw(rs.v)}}impl<M:Mdl> SubAssign for Mi<M>{fn sub_assign(&mut self,rs:Self){*self=*self-rs;}}impl<M:Mdl> Mul for Mi<M>{type Output=Self;fn mul(self,rs:Self)->Self::Output{self.mul_raw(rs.v)}}impl<M:Mdl> MulAssign for Mi<M>{fn mul_assign(&mut self,rs:Self){*self=*self*rs;}}impl<M:Mdl> Div for Mi<M>{type Output=Self;fn div(self,rs:Self)->Self::Output{debug_assert!(rs.v!=0);self*rs.iv()}}impl<M:Mdl> DivAssign for Mi<M>{fn div_assign(&mut self,rs:Self){debug_assert!(rs.v!=0);*self*=rs.iv()}}impl<M:Mdl> From<u32> for Mi<M>{fn from(value:u32)->Self{Self::new(value as u64)}}impl<M:Mdl> From<u64> for Mi<M>{fn from(value:u64)->Self{Self::new(value)}}impl<M:Mdl> From<i32> for Mi<M>{fn from(value:i32)->Self{Self::new_signed(value as i64)}}impl<M:Mdl> From<i64> for Mi<M>{fn from(value:i64)->Self{Self::new_signed(value)}}pub fn comb<M:Mdl>(size:u32)->impl Fn(usize,usize)->Mi<M>{let mut f=vec![Mi::<M>::one()];f.append(&mut (1..=size).scan(Mi::<M>::one(),|s,v|{*s*=Mi::new(v as u64);Some(*s)}).collect());let iv=f[size as usize].iv();let mut ifa=vec![iv];ifa.append(&mut (1..=size).rev().scan(iv,|s,v|{*s*=Mi::new(v as u64);Some(*s)}).collect());ifa.reverse();move |n:usize,k:usize|{if n<k{Mi::zero()}else{f[n]*ifa[k]*ifa[n-k]}}}}pub mod __numeric_0_1_0{pub mod float{use super::Numeric;use std::ops::Neg;macro_rules! impl_numeric_trait_for_float{($($t:tt)*)=>{$(impl Numeric for $t{fn max_value()->Self{std::$t::MAX}fn min_value()->Self{std::$t::MIN}})*};}impl_numeric_trait_for_float!(f32 f64);pub trait Float:Numeric+Neg<Output=Self>{fn abs(self)->Self;fn acos(self)->Self;fn asin(self)->Self;fn atan(self)->Self;fn atan2(self,other:Self)->Self;fn cbrt(self)->Self;fn ceil(self)->Self;fn cos(self)->Self;fn div_euclid(self,rs:Self)->Self;fn floor(self)->Self;fn hypot(self,other:Self)->Self;fn is_finite(self)->bool;fn is_infinite(self)->bool;fn is_nan(self)->bool;fn is_sign_negative(self)->bool;fn is_sign_positive(self)->bool;fn max(self,other:Self)->Self;fn min(self,other:Self)->Self;fn mul_add(self,a:Self,b:Self)->Self;fn powf(self,n:Self)->Self;fn powi(self,n:i32)->Self;fn rem_euclid(self,rs:Self)->Self;fn round(self)->Self;fn signum(self)->Self;fn sin(self)->Self;fn sin_cos(self)->(Self,Self);fn sqrt(self)->Self;fn tan(self)->Self;fn to_radians(self)->Self;fn pi()->Self;}macro_rules! impl_float_trait{($($t:tt)*)=>{$(impl Float for $t{fn abs(self)->Self{self.abs()}fn acos(self)->Self{self.acos()}fn asin(self)->Self{self.asin()}fn atan(self)->Self{self.atan()}fn atan2(self,other:Self)->Self{self.atan2(other)}fn cbrt(self)->Self{self.cbrt()}fn ceil(self)->Self{self.ceil()}fn cos(self)->Self{self.cos()}fn div_euclid(self,rs:Self)->Self{self.div_euclid(rs)}fn floor(self)->Self{self.floor()}fn hypot(self,other:Self)->Self{self.hypot(other)}fn is_finite(self)->bool{self.is_finite()}fn is_infinite(self)->bool{self.is_infinite()}fn is_nan(self)->bool{self.is_nan()}fn is_sign_negative(self)->bool{self.is_sign_negative()}fn is_sign_positive(self)->bool{self.is_sign_positive()}fn max(self,other:Self)->Self{self.max(other)}fn min(self,other:Self)->Self{self.min(other)}fn mul_add(self,a:Self,b:Self)->Self{self.mul_add(a,b)}fn powf(self,n:Self)->Self{self.powf(n)}fn powi(self,n:i32)->Self{self.powi(n)}fn rem_euclid(self,rs:Self)->Self{self.rem_euclid(rs)}fn round(self)->Self{self.round()}fn signum(self)->Self{self.signum()}fn sin(self)->Self{self.sin()}fn sin_cos(self)->(Self,Self){self.sin_cos()}fn sqrt(self)->Self{self.sqrt()}fn tan(self)->Self{self.tan()}fn to_radians(self)->Self{self.to_radians()}fn pi()->Self{std::$t::consts::PI}})*};}impl_float_trait!(f32 f64);}pub mod integer{use super::Numeric;use std::ops::{BitAnd,BitAndAssign,BitOr,BitOrAssign,BitXor,BitXorAssign,Rem,RemAssign,Shl,ShlAssign,Shr,ShrAssign};macro_rules! impl_numeric_trait_for_integer{($($t:tt)*)=>{$(impl Numeric for $t{fn max_value()->Self{std::$t::MAX}fn min_value()->Self{std::$t::MIN}})*};}impl_numeric_trait_for_integer!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);pub trait Integer:Numeric+Rem<Self,Output=Self>+RemAssign+Shl<i32,Output=Self>+Shl<i64,Output=Self>+Shl<u32,Output=Self>+Shl<u64,Output=Self>+Shl<usize,Output=Self>+Shr<i32,Output=Self>+Shr<i64,Output=Self>+Shr<u32,Output=Self>+Shr<u64,Output=Self>+Shr<usize,Output=Self>+ShlAssign<i32>+ShlAssign<i64>+ShlAssign<u32>+ShlAssign<u64>+ShlAssign<usize>+ShrAssign<i32>+ShrAssign<i64>+ShrAssign<u32>+ShrAssign<u64>+ShrAssign<usize>+BitAnd<Self,Output=Self>+BitOr<Self,Output=Self>+BitXor<Self,Output=Self>+BitAndAssign+BitOrAssign+BitXorAssign+std::hash::Hash+Eq+Ord{fn abs_diff(self,other:Self)->Self;fn count_ones(self)->u32;fn count_zeros(self)->u32;fn div_euclid(self,rs:Self)->Self;fn leading_ones(self)->u32;fn leading_zeros(self)->u32;fn rem_euclid(self,rs:Self)->Self;fn reverse_bits(self)->Self;fn rotate_left(self,n:u32)->Self;fn rotate_right(self,n:u32)->Self;fn trailing_ones(self)->u32;fn trailing_zeros(self)->u32;fn overflowing_add(self,rs:Self)->(Self,bool);fn overflowing_mul(self,rs:Self)->(Self,bool);fn overflowing_neg(self)->(Self,bool);fn overflowing_shl(self,rs:u32)->(Self,bool);fn overflowing_shr(self,rs:u32)->(Self,bool);fn overflowing_sub(self,rs:Self)->(Self,bool);fn saturating_add(self,rs:Self)->Self;fn saturating_mul(self,rs:Self)->Self;fn saturating_sub(self,rs:Self)->Self;fn wrapping_add(self,rs:Self)->Self;fn wrapping_mul(self,rs:Self)->Self;fn wrapping_neg(self)->Self;fn wrapping_shl(self,rs:u32)->Self;fn wrapping_shr(self,rs:u32)->Self;fn wrapping_sub(self,rs:Self)->Self;}macro_rules! impl_integer_trait{($($t:ty)*)=>{$(impl Integer for $t{fn abs_diff(self,other:Self)->Self{std::cmp::max(self,other)-std::cmp::min(self,other)}fn count_ones(self)->u32{self.count_ones()}fn count_zeros(self)->u32{self.count_zeros()}fn div_euclid(self,rs:Self)->Self{self.div_euclid(rs)}fn leading_ones(self)->u32{(!self).leading_zeros()}fn leading_zeros(self)->u32{self.leading_zeros()}fn rem_euclid(self,rs:Self)->Self{self.rem_euclid(rs)}fn reverse_bits(self)->Self{self.reverse_bits()}fn rotate_left(self,n:u32)->Self{self.rotate_left(n)}fn rotate_right(self,n:u32)->Self{self.rotate_right(n)}fn trailing_ones(self)->u32{(!self).trailing_zeros()}fn trailing_zeros(self)->u32{self.trailing_zeros()}fn overflowing_add(self,rs:Self)->(Self,bool){self.overflowing_add(rs)}fn overflowing_mul(self,rs:Self)->(Self,bool){self.overflowing_mul(rs)}fn overflowing_neg(self)->(Self,bool){self.overflowing_neg()}fn overflowing_shl(self,rs:u32)->(Self,bool){self.overflowing_shl(rs)}fn overflowing_shr(self,rs:u32)->(Self,bool){self.overflowing_shr(rs)}fn overflowing_sub(self,rs:Self)->(Self,bool){self.overflowing_sub(rs)}fn saturating_add(self,rs:Self)->Self{self.saturating_add(rs)}fn saturating_mul(self,rs:Self)->Self{self.saturating_mul(rs)}fn saturating_sub(self,rs:Self)->Self{self.saturating_sub(rs)}fn wrapping_add(self,rs:Self)->Self{self.wrapping_add(rs)}fn wrapping_mul(self,rs:Self)->Self{self.wrapping_mul(rs)}fn wrapping_neg(self)->Self{self.wrapping_neg()}fn wrapping_shl(self,rs:u32)->Self{self.wrapping_shl(rs)}fn wrapping_shr(self,rs:u32)->Self{self.wrapping_shr(rs)}fn wrapping_sub(self,rs:Self)->Self{self.wrapping_sub(rs)}})*};}impl_integer_trait!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);}pub mod one{pub trait One{fn one()->Self;}macro_rules! impl_one_integer{($($t:ty)*)=>{$(impl One for $t{fn one()->$t{1}})*};}impl_one_integer!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);macro_rules! impl_one_float{($($t:ty)*)=>{$(impl One for $t{fn one()->$t{1.0}})*};}impl_one_float!(f32 f64);}pub mod signed{use std::ops::Neg;pub trait Signed:Neg<Output=Self>+std::marker::Sized{fn is_negative(self)->bool;fn is_positive(self)->bool;}macro_rules! impl_integer_trait{($($t:ty)*)=>{$(impl Signed for $t{fn is_negative(self)->bool{self.is_negative()}fn is_positive(self)->bool{self.is_positive()}})*};}impl_integer_trait!(i8 i16 i32 i64 i128 isize);}pub mod zero{pub trait Zero{fn zero()->Self;}macro_rules! impl_zero_integer{($($t:ty)*)=>{$(impl Zero for $t{fn zero()->$t{0}})*};}impl_zero_integer!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);macro_rules! impl_zero_float{($($t:ty)*)=>{$(impl Zero for $t{fn zero()->$t{0.0}})*};}impl_zero_float!(f32 f64);}pub use float::Float;pub use integer::Integer;pub use one::One;pub use signed::Signed;pub use zero::Zero;use std::ops::{Add,AddAssign,Div,DivAssign,Mul,MulAssign,Sub,SubAssign};#[derive(Debug)]pub struct Error(pub &'static str);impl std::fmt::Display for Error{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"{}",self.0)}}impl std::error::Error for Error{}pub trait UnorderedNumeric:Add<Self,Output=Self>+Sub<Self,Output=Self>+Mul<Self,Output=Self>+Div<Self,Output=Self>+AddAssign+SubAssign+MulAssign+DivAssign+std::fmt::Debug+std::fmt::Display+Clone+Copy+PartialEq+Default+Zero+One{}pub trait Numeric:Add<Self,Output=Self>+Sub<Self,Output=Self>+Mul<Self,Output=Self>+Div<Self,Output=Self>+AddAssign+SubAssign+MulAssign+DivAssign+std::fmt::Debug+std::fmt::Display+Clone+Copy+PartialEq+PartialOrd+Default+Zero+One{fn max_value()->Self;fn min_value()->Self;}pub trait IntoFloat:Numeric{fn as_f64(self)->f64;fn as_f32(self)->f32;}impl IntoFloat for i64{fn as_f64(self)->f64{self as f64}fn as_f32(self)->f32{self as f32}}}}pub(crate) mod macros{pub mod mat{}pub mod mi{}pub mod __numeric_0_1_0{}}pub(crate) mod prl{pub use crate::__ce::crates::*;}mod preludes{pub mod mat{pub(in crate::__ce) use crate::__ce::crates::{__numeric_0_1_0 as numeric,mi};}pub mod mi{pub(in crate::__ce) use crate::__ce::crates::__numeric_0_1_0 as numeric;}pub mod __numeric_0_1_0{}}}