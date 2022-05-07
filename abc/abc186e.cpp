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

using namespace std;

#define DEBUG(var) cerr << #var << ": " << (var) << " "
#define DEBUG_EN(var) cerr << #var << ": " << (var) << endl

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
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  vector<ll> res;
  while(t--) {
    ll n, s, k;
    cin >> n >> s >> k;
    set<ll> ck;
    ck.insert(s);
    if((n-s) % k == 0) {
      res.push_back((n-s) / k);
      continue;
    }
    if(n % k == 0) {
      res.push_back(-1);
      continue;
    }
    ll t = (n / k + 1) * k % n;
    DEBUG_EN(t);
    res.push_back(lcm(t, n-s) / t * (t / k + 1));
  }
  for(auto e : res) cout << e << endl;
  return 0;
}
