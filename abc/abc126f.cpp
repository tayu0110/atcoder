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

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int m;
  ll k;
  cin >> m >> k;
  if(k >= (1<<m)) {
    cout << -1 << endl;
    return 0;
  }
  if(!m) {
    if(!k) cout << "0 0" << endl;
    else cout << -1 << endl;
  } else if(m == 1) {
    if(!k) cout << "1 1 0 0" << endl;
    else cout << -1 << endl;
  } else {
    vector<int> v((1<<m)-1);
    for(int i=0;i<(1<<m)-1;i++) {
      if(i < k) v[i] = i;
      else if(i >= k) v[i] = i+1;
    }
    vector<int> r = v;
    reverse(r.begin(), r.end());
    for(auto e : v) cout << e << " ";
    cout << k << " ";
    for(auto e : r) cout << e << " ";
    cout << k << endl;
  }
  return 0;
}
