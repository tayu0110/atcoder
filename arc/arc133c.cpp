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

#include <atcoder/all>

using namespace std;
using namespace atcoder;

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
const ld PI = acos(-1);

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  ll h, w, k;
  cin >> h >> w >> k;
  vector<ll> a(h);
  vector<ll> b(w);
  for(int i=0;i<h;i++) cin >> a[i];
  for(int i=0;i<w;i++) cin >> b[i];
  ll suma = accumulate(a.begin(), a.end(), 0LL);
  ll sumb = accumulate(b.begin(), b.end(), 0LL);
  if(suma % k != sumb % k) {
    cout << -1 << endl;
    return 0;
  }
  suma = 0, sumb = 0;
  const ll hk = (k-1) * w;
  for(int i=0;i<h;i++) {
    ll rem = hk % k;
    if(rem >= a[i]) suma += hk - (rem - a[i]);
    else suma += hk - rem - (k - a[i]);
  }
  const ll wk = (k-1) * h;
  for(int i=0;i<w;i++) {
    ll rem = wk % k;
    if(rem >= b[i]) sumb += wk - (rem - b[i]);
    else sumb += wk - rem - (k - b[i]);
  }
  cout << min(suma, sumb) << endl;
  return 0;
}
