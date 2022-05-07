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

using namespace std;

#define DEBUG(var) cerr << #var << ": " << (var) << " "
#define DEBUG_EN(var) cerr << #var << ": " << (var) << endl

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
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  for(int i=1;i<=t;i++) {
    int n;
    cin >> n;
    vector<ll> a(n), b(n);
    ll sum = 0;
    for(int j=0;j<n;j++) {
      if(j >= 30) {
        if(j == 32) a[j] = 200;
        else if(j == 64) a[j] = 300;
        else a[j] = j;
      } else {
        ll k = 1 << j;
        a[j] = k;
      }
      if(j) cout << " "; cout << a[j];
    }
    cout << endl;
    for(int j=0;j<n;j++) {
      int k;
      cin >> k;
      b[j] = k;
      sum += a[j] + k;
    }
    sort(b.begin(), b.end(), greater<ll>());
    sum /= 2;
    vector<ll> c, d;
    ll sc = 0, sd = 0;
    for(int j=0;j<n;j++) {
      if(sc < sd) {
        c.push_back(b[j]);
        sc += b[j];
      } else {
        d.push_back(b[j]);
        sd += b[j];
      }
    }
    vector<ll> res;
    if(sc < sd) res = d;
    else res = c;
    ll diff = sum - max(sc, sd);
    for(int j=0;j<30;j++) {
      if(diff & (1LL << j)) res.push_back(a[j]);
    }
    for(int j=0;j<res.size();j++) {
      if(j) cout << " "; cout << res[j];
    }
    cout << endl;
  }
  return 0;
}
