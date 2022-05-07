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
const int mod = 2019;
int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  string s;
  cin >> s;
  int n = s.length();
  reverse(s.begin(), s.end());
  int t = 1;
  vector<int> p(n);
  for(int i=0;i<n;i++) {
    int now = s[i] - '0';
    p[i] = now * t % mod;
    if(i) p[i] = (p[i] + p[i-1]) % mod;
    t *= 10;
    t %= mod;
  }
  map<int, ll> mp;
  mp[0]++;
  for(int i=0;i<n;i++) mp[p[i]]++;
  ll ans = 0;
  for(auto [fi, se] : mp) {
    ans += se * (se-1) / 2;
  }
  cout << ans << endl;
  return 0;
}
