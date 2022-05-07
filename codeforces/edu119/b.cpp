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
    int w, h;
    cin >> w >> h;
    int k1;
    cin >> k1;
    vector<ll> x0(k1);
    for(int i=0;i<k1;i++) cin >> x0[i];
    int k2;
    cin >> k2;
    vector<ll> xh(k2);
    for(int i=0;i<k2;i++) cin >> xh[i];
    int k3;
    cin >> k3;
    vector<ll> y0(k3);
    for(int i=0;i<k3;i++) cin >> y0[i];
    int k4;
    cin >> k4;
    vector<ll> yw(k4);
    for(int i=0;i<k4;i++) cin >> yw[i];
    ll s1 = (x0[k1-1] - x0[0]) * h;
    ll s2 = (xh[k2-1] - xh[0]) * h;
    ll s3 = (y0[k3-1] - y0[0]) * w;
    ll s4 = (yw[k4-1] - yw[0]) * w;
    res.push_back(max({s1, s2, s3, s4}));
  }
  for(auto e : res) cout << e << endl;
  return 0;
}
