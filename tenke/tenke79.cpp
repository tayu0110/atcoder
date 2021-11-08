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
  int h, w;
  cin >> h >> w;
  vector<vector<ll>> a(h, vector<ll>(w, 0)), b(h, vector<ll>(w, 0));
  for(int i=0;i<h;i++) for(int j=0;j<w;j++) cin >> a[i][j];
  for(int i=0;i<h;i++) for(int j=0;j<w;j++) cin >> b[i][j];
  ll ans = 0;
  for(int i=0;i<h;i++) for(int j=0;j<w;j++) {
    ll k = a[i][j], l = b[i][j];
    ll t = a[i][j] - b[i][j];
    if(i == h-1 || j == w-1) {
      if(!t) continue;
      else {
        cout << "No" << endl;
        return 0;
      }
    } else {
      for(int p=0;p<2;p++) for(int q=0;q<2;q++) a[i+p][j+q] -= t;
      ans += abs(t);
    }
  }
  cout << "Yes" << endl;
  cout << ans << endl;
  return 0;
}
