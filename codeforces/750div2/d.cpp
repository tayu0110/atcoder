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
    int n;
    cin >> n;
    vector<ll> a(n);
    for(int i=0;i<n;i++) cin >> a[i];
    vector<pll> v(n);
    for(int i=0;i<n;i++) v[i] = {abs(a[i]), i};
    sort(v.begin(), v.end());
    vector<pll> p, q;
    for(int i=0;i<n;i++) {
      if(i % 2) q.push_back(v[i]);
      else p.push_back(v[i]);
    }
    vector<pll> r(n);
    for(int i=0;i<q.size();i++){
      auto [qf, qs] = q[i];
      auto [pf, ps] = p[i];
      if(i == q.size()-1 && n % 2) {
        auto [pf2, ps2] = p[i+1];
        r[ps] = {qf, ps};
        r[ps2] = {qf, ps2};
        r[qs] = {-(pf+pf2), qs};
      } else {
        r[ps] = {qf, ps};
        r[qs] = {-pf, qs};
      }
    }
    vector<int> b(n);
    for(auto &[f, s] : r) {
      if((f < 0 && a[s] < 0) || (f > 0 && a[s] < 0)) f *= -1;
      b[s] = f;
    }
    for(int i=0;i<n;i++) {
      cout << b[i];
      if(i == n-1) cout << endl;
      else cout << " ";
    }
  }
  return 0;
}
