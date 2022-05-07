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
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  vector<ll> a(n+1), c(n+m+1);
  for(int i=0;i<n+1;i++) cin >> a[i];
  for(int i=0;i<n+m+1;i++) cin >> c[i];
  reverse(a.begin(), a.end());
  reverse(c.begin(), c.end());
  vector<ll> b;
  for(int i=0;i<m+1;i++) {
    int k = c[i] / a[0];
    b.push_back(k);
    for(int j=0;j<n+1;j++) {
      c[i+j] -= k * a[j];
    }
  }
  reverse(b.begin(), b.end());
  for(int i=0;i<m+1;i++) {
    if(i) cout << " "; cout << b[i];
  }
  cout << endl;
  return 0;
}
