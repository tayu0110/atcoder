#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<deque>
#include<set>
#include<stack>
#include<numeric>
#include<complex>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>
#include<cassert>
#include<cstdarg>

using namespace std;

#define DEBUG(var) cout << #var << ": " << var << " ";
#define DEBUG_EN(var) cout << #var << ": " << var << endl;

struct Edge {
  int to;
  long long weight;
  Edge() : to(0), weight(0) {}
  Edge(int to, long long weight) : to(to), weight(weight) {}
  Edge(const Edge& e) {
    to = e.to;
    weight = e.weight;
  }
  bool operator>(const Edge &e) const { return weight > e.weight; }
  bool operator<(const Edge &e) const { return weight < e.weight; }
  bool operator==(const Edge &e) const { return weight == e.weight; }
  bool operator<=(const Edge &e) const { return weight <= e.weight; }
  bool operator>=(const Edge &e) const { return weight >= e.weight; }
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;
using heap = priority_queue<int, vector<int>, greater<int>>;

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;

template <typename T>
class Complex {
  T r, i;
  const T eps = 1e-6;
public:
  Complex() : r(0), i(0) {}
  Complex(T real, T imag) : r(real), i(imag) {}
  Complex(Complex& c) : r(c.real()), i(c.imag()) {}
  Complex(const Complex& c) : r(c.real()), i(c.imag()) {}
  ~Complex() = default;
  T real() { return r; }
  T imag() { return i; }
  T real() const { return r; }
  T imag() const { return i; }
  constexpr Complex<T>& operator=(const T& rhs) { r = rhs; i = 0; return *this; }
  constexpr Complex<T>& operator=(const Complex<T>& rhs) { r = rhs.real(); i = rhs.imag(); return *this; }
  template<typename X>
  constexpr Complex<T>& operator=(const Complex<X>& rhs) { r = rhs.real(); i = rhs.imag(); return *this; }
  constexpr Complex<T>& operator+=(const T& rhs) { r += rhs; return *this; }
  template<typename X>
  constexpr Complex<T>& operator+=(const Complex<X>& rhs) { r += rhs.real(); i += rhs.imag(); return *this; }
  constexpr Complex<T>& operator-=(const T& rhs) { r -= rhs; return *this; }
  template<typename X>
  constexpr Complex<T>& operator-=(const Complex<X>& rhs) { r -= rhs.real(); i -= rhs.imag(); return *this; }
  constexpr Complex<T>& operator*=(const T& rhs) { r *= rhs; i *= rhs; return *this; }
  template<typename X>
  constexpr Complex<T>& operator*=(const Complex<X>& rhs) {
    T nr = r*rhs.real() - i*rhs.imag();
    T ni = i*rhs.real() + r*rhs.imag();
    r = nr, i = ni;
    return *this;
  }
  constexpr Complex<T>& operator/=(const T& rhs) { r /= rhs; i /= rhs; return *this; }
  template<typename X>
  constexpr Complex<T>& operator/=(const Complex<X>& rhs) {
    T rr = rhs.real(), ri = rhs.imag();
    T nr = (r*rr + i*ri) / (rr*rr + ri*ri);
    T ni = (i*rr - r*ri) / (rr*rr + ri*ri);
    r = nr, i = ni;
    return *this;
  }
  constexpr Complex<T> operator+() { return *this; }
  constexpr Complex<T> operator-() { return Complex<T>(-r, -i); }
  constexpr Complex<T> operator+(const Complex<T>& rhs) { return Complex<T>(*this) += rhs; }
  constexpr Complex<T> operator+(const T& rhs) { return Complex<T>(*this) += rhs; }
  constexpr Complex<T> operator-(const Complex<T>& rhs) { return Complex<T>(*this) -= rhs; }
  constexpr Complex<T> operator-(const T& rhs) { return Complex<T>(*this) -= rhs; }
  constexpr Complex<T> operator*(const Complex<T>& rhs) { return Complex<T>(*this) *= rhs; }
  constexpr Complex<T> operator*(const T& rhs) { return Complex<T>(*this) *= rhs; }
  constexpr Complex<T> operator/(const Complex<T>& rhs) { return Complex<T>(*this) /= rhs; }
  constexpr Complex<T> operator/(const T& rhs) { return Complex<T>(*this) /= rhs; }
  constexpr bool operator==(Complex<T>& rhs) { return (std::abs(r-rhs.real()) <= eps) && (std::abs(i-rhs.imag()) <= eps); }
  constexpr bool operator!=(Complex<T>& rhs) { return (std::abs(r-rhs.real()) > eps) || (std::abs(i-rhs.imag()) > eps); }
  constexpr bool operator<(Complex<T>& rhs) { return (std::abs(r-rhs.real()) <= eps) ? i < rhs.imag() : r < rhs.real(); }
  constexpr bool operator>(Complex<T>& rhs) { return (std::abs(r-rhs.real()) <= eps) ? i > rhs.imag() : r > rhs.real(); }
  constexpr bool operator<=(Complex<T>& rhs) { return (*this == rhs) || (*this < rhs); }
  constexpr bool operator>=(Complex<T>& rhs) { return (*this == rhs) || (*this > rhs); }
  constexpr bool operator==(const Complex<T>& rhs) const { return (std::abs(r-rhs.real()) <= eps) && (std::abs(i-rhs.imag()) <= eps); }
  constexpr bool operator!=(const Complex<T>& rhs) const { return (std::abs(r-rhs.real()) > eps) || (std::abs(i-rhs.imag()) > eps); }
  constexpr bool operator<(const Complex<T>& rhs) const { return (std::abs(r-rhs.real()) <= eps ? i < rhs.imag() : r < rhs.real()); }
  constexpr bool operator>(const Complex<T>& rhs) const { return (std::abs(r-rhs.real()) <= eps ? i > rhs.imag() : r > rhs.real()); }
  constexpr bool operator<=(const Complex<T>& rhs) const { return (*this == rhs) || (*this < rhs); }
  constexpr bool operator>=(const Complex<T>& rhs) const { return (*this == rhs) || (*this > rhs); }
  T norm() { return r*r + i*i; }
  T abs() { return sqrtl(this->norm()); }
  T arg() { return atan2(i, r); }
  constexpr Complex<T> conj() { return Complex<T>(r, -i); }
  Complex<T> polar(const T& rho, const T& theta = 0) { return Complex<T>(rho*cos(theta), rho*sin(theta)); }
  friend ostream &operator<<(ostream &os, const Complex<T> &c) {os << "(" << c.real() << ", " << c.imag() << ")"; return os;}
};

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  // vector<complex<double>> s(n), t(n);
  vector<Complex<int>> s(n), t(n);
  for(int i=0;i<n;i++) {
    int a, b;
    cin >> a >> b;
    s[i] = {a, b};
  }
  for(int i=0;i<n;i++) {
    int c, d;
    cin >> c >> d;
    t[i] = {c, d};
  }
  if(n == 1) {
    cout << "Yes" << endl;
    return 0;
  }
  // using vc = vector<complex<double>>;
  using vc = vector<Complex<int>>;
  for(int i=0;i<n;i++) for(int j=0;j<n;j++) {
    if(i == j) continue;
    auto a = t[i], b = t[j];
    if((s[1]-s[0]).norm() != (b-a).norm()) continue;
    vc na = s, nb = t;
    // auto f = [&](vc& p, complex<double> q, complex<double> r) {
    //   for(int i=0;i<n;i++) p[i] -= q;
    //   for(int i=0;i<n;i++) p[i] *= r;
    //   sort(p.begin(), p.end(), [](complex<double> lhs, complex<double> rhs) {
    //     if(lhs.real() == rhs.real()) return lhs.imag() < rhs.imag();
    //     return lhs.real() < rhs.real();
    //   });
    // };
    auto f = [&](vc& p, Complex<int> q, Complex<int> r) {
      for(int i=0;i<n;i++) p[i] -= q;
      for(int i=0;i<n;i++) p[i] *= r;
      sort(p.begin(), p.end());
    };
    f(na, s[0], b-a);
    f(nb, a, s[1]-s[0]);    
    bool flag = true;
    for(int i=0;i<n;i++) if(na[i] != nb[i]) flag = false;
    if(flag) {
      cout << "Yes" << endl;
      return 0;
    }
  }
  cout << "No" << endl;
  return 0;
}
