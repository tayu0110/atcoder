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
  vector<ll> res;
  while(t--) {
    int n, m;
    cin >> m >> n;
    vector<vector<ll>> p(m, vector<ll>(n, 0));
    for(int i=0;i<m;i++) for(int j=0;j<n;j++) cin >> p[i][j];
    ll l = 0, r = 1001001001;
    vector<vector<ll>> v(m, vector<ll>(n, 0));
    while(r-l > 1) {
      ll mid = (r+l) / 2;
      vector<int> a(n, 0);
      vector<pii> b(m);
      for(int i=0;i<m;i++) for(int j=0;j<n;j++) {
        if(p[i][j] >= mid) v[i][j] = 1;
        else v[i][j] = 0;
        a[j] += v[i][j];
        b[i] = {b[i].first + v[i][j], i};
      }
      if(*min_element(a.begin(), a.end()) == 0) {
        r = mid;
        continue;
      }
      sort(b.begin(), b.end(), greater<pii>());
      if(b[0].first == 1) {
        r = mid;
        continue;
      } else {
        l = mid;
      }
    }
    res.push_back(l);
  }
  for(auto e : res) cout << e << endl;
  return 0;
}
