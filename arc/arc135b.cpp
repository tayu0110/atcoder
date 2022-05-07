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
const ld PI = acos(-1);

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<ll> s(n);
  for(int i=0;i<n;i++) cin >> s[i];
  vector<ll> a(n+2);
  a[0] = 0;
  a[1] = 0;
  a[2] = 0;
  vector<ll> mn(3, 0);
  for(int i=3;i<n+2;i++) {
    ll diff = s[i-2] - s[i-3];
    a[i] = a[i-3] + diff;
    mn[i % 3] = min(mn[i%3], a[i]);
  }
  for(int i=0;i<n+2;i++) a[i] -= mn[i%3];
  vector<ll> ns(n);
  ll diff = -1;
  for(int i=0;i<n;i++) {
    ns[i] = a[i] + a[i+1] + a[i+2];
    if(ns[i] > s[i]) {
      cout << "No" << endl;
      return 0;
    }
    if(diff < 0) {
      diff = s[i] - ns[i];
    } else {
      if(diff != s[i] - ns[i]) {
        cout << "No" << endl;
        return 0;
      }
    }
  }
  cout << "Yes" << endl;
  for(int i=0;i<n+2;i++) {
    if(i) cout << " ";
    ll out = a[i];
    if(i % 3 == 0) out += diff;
    cout << out;
  }
  cout << endl;
  return 0;
}
