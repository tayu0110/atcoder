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
  int n;
  cin >> n;
  vector<int> a(n);
  for(int i=0;i<n;i++) {
    cin >> a[i];
  }
  ll ans = 0;
  map<int, ll> mp;
  for(int i=0;i<n;i++) mp[a[i]]++;
  for(auto [f, s] : mp) {
    for(int i=1;i*i<=f;i++) {
      if(f%i) continue;
      if(mp.find(i) == mp.end()) continue;
      int j = f / i;
      if(mp.find(j) == mp.end()) continue;
      if(i == j) ans += s * mp[i] * mp[j];
      else ans += s * mp[i] * mp[j] * 2;
    }
  }
  cout << ans << endl;
  return 0;
}
