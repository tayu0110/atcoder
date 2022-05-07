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
  vector<ll> a(n), b(n);
  for(int i=0;i<n;i++) cin >> a[i];
  for(int i=0;i<n;i++) cin >> b[i];
  priority_queue<pll> nt;
  for(int i=0;i<n;i++) nt.push({b[i], i});
  ll ans = 0;
  while(!nt.empty()) {
    auto [v, idx] = nt.top();
    nt.pop();
    int prev = (idx + n - 1) % n;
    int next = (idx + 1) % n;
    if(v < b[prev] + b[next]) continue;
    if(v % (b[prev] + b[next]) >= a[idx]) {
      ans += v / (b[prev] + b[next]);
      v %= (b[prev] + b[next]);
      b[idx] = v;
      nt.push({v, idx});
    } else {
      if((v - a[idx]) % (b[prev] + b[next]) != 0) {
        cout << -1 << endl;
        return 0;
      }
      ans += (v - a[idx]) / (b[prev] + b[next]);
      b[idx] = a[idx];
    }
  }
  for(int i=0;i<n;i++) if(a[i] != b[i]) {
    cout << -1 << endl;
    return 0;
  }
  cout << ans << endl;
  return 0;
}
