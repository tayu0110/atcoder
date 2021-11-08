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
  vector<int> res;
  while(t--) {
    int n;
    ll h;
    cin >> n >> h;
    vector<ll> a(n);
    for(int i=0;i<n;i++) cin >> a[i];
    sort(a.begin(), a.end(), greater<ll>());
    ll p = a[0], q = a[1];
    ll s = h / (p+q);
    h -= s * (p+q);
    if(h == 0) {
      res.push_back(s*2);
    } else if(h > p) {
      res.push_back(s*2+2);
    } else {
      res.push_back(s*2+1);
    }
  }
  for(auto e : res) cout << e << endl;
  return 0;
}
