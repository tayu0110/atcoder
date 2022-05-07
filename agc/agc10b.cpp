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
  ll n;
  cin >> n;
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  ll sum = accumulate(a.begin(), a.end(), 0LL);
  if(sum % (n * (n+1) / 2) != 0) {
    cout << "NO" << endl;
    return 0;
  }
  ll k = sum / (n * (n + 1) / 2);
  DEBUG_EN(k);
  a.push_back(a[0]);
  vector<ll> d(n);
  for(int i=0;i<n;i++) d[i] = a[i+1] - a[i] - k;
  ll t = 0;
  for(int i=0;i<n;i++) {
    if(d[i] > 0) {
      cout << "NO" << endl;
      return 0;
    }
    if(d[i] < 0) t += abs(d[i]) / n;
  }
  DEBUG_EN(t);
  cout << (k == t ? "YES" : "NO") << endl;
  return 0;
}
