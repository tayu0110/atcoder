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
  int n;
  cin >> n;
  vector<ll> a(2*n);
  for(int i=0;i<2*n;i++) cin >> a[i];
  priority_queue<pll> nt;
  for(int i=0;i<2*n;i++) nt.push({a[i], i});
  vector<bool> t(2*n, false);
  for(int i=0;i<n;i++) {
    auto [b, idx] = nt.top(); nt.pop();
    t[idx] = true;
  }
  stack<int> ck;
  vector<char> res(2*n);
  for(int i=0;i<2*n;i++) {
    if(ck.empty()) ck.push(i);
    else {
      int idx = ck.top();
      if(t[idx] == t[i]) ck.push({i});
      else {
        ck.pop();
        res[idx] = '(';
        res[i] = ')';
      }
    }
  }
  for(int i=0;i<2*n;i++) cout << res[i];
  cout << endl;
  return 0;
}
