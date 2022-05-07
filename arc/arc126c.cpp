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
const ld PI = 3.141592653589793238462643383;
bool check(int d, ll k, vector<int> &a) {
  int sum = 0;
  for(int i=0;i<a.size();i++) {
    sum += (d - a[i] % d) % d;
  }
  return sum <= k;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  ll k;
  cin >> n >> k;
  vector<int> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  sort(a.begin(), a.end());
  {
    ll sum = 0;
    for(int i=0;i<n;i++) sum += a[n-1] - a[i];
    if(k >= sum) {
      k -= sum;
      cout << a[n-1] + k / n << endl;
      return 0;
    }
  }
  vector<ll> sum(2*a[n-1]+1, 0);
  vector<ll> cnt(2*a[n-1]+1, 0);
  for(int i=0;i<n;i++) {
    sum[a[i]] += a[i];
    cnt[a[i]]++;
  }
  for(int i=0;i<sum.size()-1;i++) {
    sum[i+1] += sum[i];
    cnt[i+1] += cnt[i];
  }
  // print_with_space(sum);
  // print_with_space(cnt);
  for(int g=a[n-1];g>0;g--) {
    ll need = 0;
    for(int i=1;g*(i-1)<a[n-1];i++) {
      int nc = cnt[g*i] - cnt[g*(i-1)];
      ll s = sum[g*i] - sum[g*(i-1)];
      need += (ll)g * i * nc - s;
      // DEBUG(g*i);DEBUG(g*(i-1));
      // DEBUG(nc);DEBUG_EN(s);
    }
    // DEBUG(g);DEBUG_EN(need);
    if(need <= k) {
      cout << g << endl;
      return 0;
    }
  }
  return 0;
}
