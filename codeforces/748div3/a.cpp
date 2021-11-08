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
  int t;
  cin >> t;
  vector<tuple<ll, ll, ll>> res; 
  while(t--) {
    int a, b, c;
    cin >> a >> b >> c;
    int mx = max(a, max(b, c));
    int cnt = (mx == a) + (mx == b) + (mx == c);
    if(cnt > 1) {
      mx++;
      res.push_back({mx-a, mx-b, mx-c});
    } else {
      if(a == mx) a = 0;
      else a = mx+1 - a;
      if(b == mx) b = 0;
      else b = mx+1 - b;
      if(c == mx) c = 0;
      else c = mx+1 - c;
      res.push_back({a, b, c});
    }
  }
  for(auto [a, b, c] : res) cout << a << " " << b << " " << c << endl;
  return 0;
}
