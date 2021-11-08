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
  while(t--) {
    ll r, g, b;
    cin >> r >> g >> b;
    vector<ll> p = {r, g, b};
    sort(p.begin(), p.end());
    r = p[0], g = p[1], b = p[2];
    ll ans = INF;
    if((g - r) % 3 == 0) ans = min(ans, g);
    if((b - r) % 3 == 0) ans = min(ans, b); 
    if((b - g) % 3 == 0) ans = min(ans, b);
    if(ans == INF) cout << -1 << endl;
    else cout << ans << endl;
  }
  return 0;
}