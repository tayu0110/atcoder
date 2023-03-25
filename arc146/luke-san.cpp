// #pragma GCC target("avx2")
// #pragma GCC optimize("O3")
// #pragma GCC optimize("unroll-loops")
// #include "bits/stdc++.h"
#include <iostream>
#include <vector>
#include <utility>
#include <queue>
#include <functional>
#include <cmath>
#include <algorithm>


#define rep(i,n) for(ll i=0;i<(n);++i)
#define ALL(x) x.begin(),x.end()
#define BACK(x) x.rbegin(),x.rend()
#define MOD1 1000000007
#define MOD2 998244353
#define INF (LLONG_MAX / 2)
#define FLOAT_ANS setprecision(30)
#define TORAD(x) (x*acos(-1)/180.0)
#define TODEG(x) (x*180/acos(-1))

using namespace std;
using ll = long long;
using ull = unsigned long long;

template<typename T> // T:重み
using p_que = priority_queue<T,vector<T>,greater<T>>;

template<typename T>
bool chmin(T& a,T b){if(a>b){a=b;return true;}return false;}

template<typename T>
bool chmax(T& a,T b){if(a<b){a=b;return true;}return false;}

ll modpow(ll a, ll n, ll mod) {ll res=1;while (n>0) {if(n&1)res=(res*(a%mod))%mod;a=((a%mod)*(a%mod))%mod;n>>=1;}return res;}

template<typename T>
void RotateVec2(vector<vector<T>>&v){ll h=v.size();ll w=v[0].size();vector<vector<T>>t(w,vector<T>(h));rep(i,h){rep(j,w){t[j][h-i-1]=v[i][j];}}v=t;}

bool InRange(ll x, ll mn, ll mx){return (mn <= x && x <= mx);}

template<typename T>
vector<T>&merged(vector<T>&a,vector<T>&b) {vector<T>res;merge(a.begin(),a.end(),b.begin(),b.end(),back_inserter(res));return res;}

void print(){ cout << endl; }
template <class Head, class... Tail>
void print(Head&& head, Tail&&... tail)
{ cout << head << " "; print(forward<Tail>(tail)...); }

struct UnionFind{
    vector<ll>tree,data;
    UnionFind(ll x):tree(x, -1),data(x){}
    ll root(ll x){if(tree[x]<0) return x;return tree[x]=root(tree[x]);}
    bool same(ll x,ll y){return root(x)==root(y);}
    ll size(ll x){return -tree[root(x)];}
    void unite(ll x,ll y){x=root(x),y=root(y);if(x==y)return;if(size(x)<size(y))swap(x,y);tree[x]+=tree[y];tree[y]=x;data[x]+=data[y];}
};

template<typename T>
struct SegTree{
    ll n;T e;vector<T>tree;function<T(T,T)>f,add;
    SegTree(ll n_,function<T(T,T)>f_,T e_=0,function<T(T,T)>add_=[](T next,T old){return next;}):e(e_),f(f_),add(add_){ll x=1;while(x<n_)x*=2;n=x;tree.assign(n*2,e);}
    void update(ll idx,T x){idx+=n-1;tree[idx]=add(x,tree[idx]);while(idx){idx=(idx-1)/2;tree[idx]=f(tree[idx*2+1],tree[idx*2+2]);}}
    T query(ll x,ll y){return query_sub(x,y,0,n,0);}
    T query_sub(ll x,ll y,ll l,ll r,ll k){if(r<=x||y<=l)return e;if(x<=l&&r<=y)return tree[k];T c1=query_sub(x,y,l,(l+r)/2,k*2+1);T c2=query_sub(x,y,(l+r)/2,r,k*2+2);return f(c1,c2);}
    T get(ll idx){return tree[idx+n-1];}
};

template<std::uint_fast64_t Modulus> class modint {
    using u64 = std::uint_fast64_t;
public:
    u64 a;
    constexpr modint(const u64 x = 0) noexcept : a(x % Modulus) {}
    constexpr u64 &value() noexcept { return a; }
    constexpr const u64 &value() const noexcept { return a; }
    constexpr modint operator+(const modint rhs) const noexcept {return modint(*this) += rhs;}
    constexpr modint operator-(const modint rhs) const noexcept {return modint(*this) -= rhs;}
    constexpr modint operator*(const modint rhs) const noexcept {return modint(*this) *= rhs;}
    constexpr modint operator/(const modint rhs) const noexcept {return modint(*this) /= rhs;}
    constexpr modint &operator+=(const modint rhs) noexcept {a += rhs.a;if (a >= Modulus) {a -= Modulus;}return *this;}
    constexpr modint &operator-=(const modint rhs) noexcept {if (a < rhs.a) {a += Modulus;}a -= rhs.a;return *this;}
    constexpr modint &operator*=(const modint rhs) noexcept {a = a * rhs.a % Modulus;return *this;}
    constexpr modint &operator/=(modint rhs) noexcept {u64 exp = Modulus - 2;while (exp) {if (exp % 2) {*this *= rhs;}rhs *= rhs;exp /= 2;}return *this;}
    constexpr modint &operator=(u64 x){ a = x % Modulus; return *this; }
};

struct Vec2 {
    ll x, y;
    Vec2() {}
    Vec2(ll x_, ll y_) {
        x = x_;
        y = y_;
    }

    double length() { return sqrt((double)x*x+y*y); };
    Vec2 operator-(Vec2 a) { return Vec2(*this) -= a; }
    Vec2 operator+(Vec2 a) { return Vec2(*this) += a; }
    ll operator*(Vec2 a) { return x*a.y+y*a.x; }
    Vec2 operator*(ll a) { return Vec2(*this) *= a; }
    Vec2 operator/(ll a) { return Vec2(*this) /= a; }
    Vec2 &operator+=(const Vec2 a) { x += a.x; y += a.y; return *this; }
    Vec2 &operator-=(const Vec2 a) { x -= a.x; y -= a.y; return *this; }
    Vec2 &operator-=(const ll a) { x -= a; y -= a; return *this; }
    Vec2 &operator*=(const ll a) { x *= a; y *= a; return *this; }
    Vec2 &operator/=(const ll a) { x /= a; y /= a; return *this; }
    friend ostream& operator<< (ostream& stream, const Vec2& x);
    bool operator==(const Vec2 a) const { return (x==a.x and y==a.y); }
    bool operator<(const Vec2 a) const 
    {
        // return y*a.x < x*a.y; // 偏角
        // return make_pair(x,y) < make_pair(a.x, a.y);
        return make_pair(x,-y) < make_pair(a.x, -a.y);
    }
};


ostream& operator<< (ostream& stream, const Vec2& x) {
    string s = "(" + to_string(x.x) + ", " + to_string(x.y) + ")";
    stream << s;
    return stream;
}


// MAIN PROGRAM ------------

int main(){
    ll n, m, k;
    cin >> n >> m >> k;
    vector<ll>a(n);
    rep(i, n) cin >> a[i];
    sort(BACK(a));
    ll ans = 0;
    ll target = 1ll << 40;
    while(target) {
        vector<Vec2>needs;
        rep(i, n) {
            ll t = max(0ll, target-(a[i]%(target*2)));
            needs.push_back({ t, (a[i]+t) % target }); // 必要数 / 足した後のtarget以下のビット
        }
        sort(ALL(needs)); // xは昇順, yは降順
        ll need = 0;
        rep(i, k) need += needs[i].x;
        if (need <= m) {
            rep(i, k) a[i] = needs[i].y;
            n = k;
            m -= need;
            ans += target;
        }
        
        target /= 2;
    }
    cout << ans << endl;
}
