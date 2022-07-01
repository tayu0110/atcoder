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
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>
#include<cassert>

using namespace std;

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

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  string s;
  cin >> s;
  vector<int> m, b;
  for(int i=0;i<n;i++) {
    if(s[i] == 'o') m.push_back(i);
    else b.push_back(i);
  }
  ll ans = 0;
  for(auto e : m) {
    auto it = lower_bound(b.begin(), b.end(), e);
    if(it == b.end()) continue;
    ans += max(0LL, (ll)(n - *it));
  }
  for(auto e : b) {
    auto it = lower_bound(m.begin(), m.end(), e);
    if(it == m.end()) continue;
    ans += max(0LL, (ll)(n - *it));
  }
  cout << ans << endl;
  return 0;
}