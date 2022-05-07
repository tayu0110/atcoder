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
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  string s;
  int q;
  cin >> s >> q;
  vector<char> res;
  while(q--) {
    ll t, k;
    cin >> t >> k;
    k--;
    if(!t) {
      res.push_back(s[k]);
      continue;
    }
    ll nk = k;
    vector<int> p;
    while(t--) {
      if(nk % 2) p.push_back(1);
      else p.push_back(0);
      nk /= 2;
      if(!nk) break;
    }
    reverse(p.begin(), p.end());
    char now = s[nk];
    if(t >= 0) {
      if(now == 'A') {
        if(t % 3 == 0) now = 'A';
        else if(t % 3 == 1) now = 'B';
        else now = 'C'; 
      } else if(now == 'B') {
        if(t % 3 == 0) now = 'B';
        else if(t % 3 == 1) now = 'C';
        else now = 'A';
      } else {
        if(t % 3 == 0) now = 'C';
        else if(t % 3 == 1) now = 'A';
        else now = 'B';
      }
    }
    for(auto e : p) {
      if(!e) {
        if(now == 'A') now = 'B';
        else if(now == 'B') now = 'C';
        else now = 'A';
      } else {
        if(now == 'A') now = 'C';
        else if(now == 'B') now = 'A';
        else now = 'B';
      }
    }
    res.push_back(now);
  }
  for(auto e : res) cout << e << endl;
  return 0;
}
