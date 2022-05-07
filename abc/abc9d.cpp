#include <iostream>
#include <iomanip>
#include <string>
#include <vector>
#include <algorithm>
#include <utility>
#include <tuple>
#include <map>
#include <queue>
#include <deque>
#include <set>
#include <stack>
#include <numeric>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <cmath>
#include <cassert>

#include <atcoder/all>

using namespace std;
using namespace atcoder;

#define DEBUG(var) cerr << #var << ": " << var << " "
#define DEBUG_EN(var) cerr << #var << ": " << var << endl

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;
template<typename T>
struct Matrix {
  int row;
  int column;
  vector<vector<T>> t;
  Matrix() : row{0}, column{0}, t{vector<vector<T>>(0, vector<T>(0, 0))} {}
  Matrix(int r, int c) : row{r}, column{c}, t{vector<vector<T>>(r, vector<T>(c, 0))} {}
  Matrix(vector<vector<T>> v) : row{(int)v.size()}, column{(int)v[0].size()}, t{v} {}
  Matrix(const Matrix &m) : row{m.row}, column{m.column}, t{m.t} {}
  constexpr Matrix operator+(const Matrix &m) {
    assert(row == m.row && column == m.column);
    Matrix res(*this);
    for(int i=0;i<row;i++) for(int j=0;j<column;j++) res[i][j] = t[i][j] ^ m[i][j];
    return res;
  }
  constexpr Matrix operator-(const Matrix &m) {
    assert(row == m.row && column == m.column);
    Matrix res(*this);
    for(int i=0;i<row;i++) for(int j=0;j<column;j++) res[i][j] = t[i][j] - m[i][j];
    return res;
  }
  constexpr Matrix operator*(const Matrix &m) {
    assert(column == m.row);
    Matrix res(row, m.column);
    for(int i=0;i<row;i++) for(int j=0;j<m.column;j++) for(int k=0;k<m.row;k++) res.t[i][j] ^= t[i][k] & m.t[k][j];
    return res;
  }
  constexpr Matrix &operator+=(const Matrix &m) {return *this = *this + m;}
  constexpr Matrix &operator-=(const Matrix &m) {return *this = *this - m;}
  constexpr Matrix &operator*=(const Matrix &m) {return *this = *this * m;}
  void set(int r, int c, T val) {t[r][c] = val;}
  T get(int r, int c) {return t[r][c];}
  Matrix pow(ll p) {
    assert(p > 0);
    if(p == 1) return *this;
    Matrix res = this->pow(p/2);
    res *= res;
    if(p % 2) res *= *this;
    return res;
  }
private:
  Matrix e(int sz) {
    Matrix res(sz, sz);
    for(int i=0;i<sz;i++) res.t[i][i] = 1;
  }
};
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int k;
  ll m;
  cin >> k >> m;
  vector<ll> a(k), c(k);
  for(int i=0;i<k;i++) cin >> a[i];
  for(int i=0;i<k;i++) cin >> c[i];
  if(m <= k) {
    cout << a[m-1] << endl;
    return 0;
  }
  Matrix<ll> mt(k, k);
  for(int i=0;i<k;i++) mt.set(0, i, c[i]);
  for(int i=0;i<k-1;i++) mt.set(i+1, i, (1LL<<33)-1);
  Matrix<ll> mt2(k, 1);
  for(int i=0;i<k;i++) mt2.set(k-1-i, 0, a[i]);
  cout << (mt.pow(m-k) * mt2).get(0, 0) << endl;
  return 0;
}
