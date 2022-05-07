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
  ll a, n;
  cin >> a >> n;
  int cnt = 0;
  vector<ll> t = {1};
  set<ll> ck;
  while(true) {
    vector<ll> tmp;
    if(t.empty()) {
      cout << -1 << endl;
      return 0;
    }
    for(auto e : t) {
      if(e == n) {
        cout << cnt << endl;
        return 0;
      }
      if(ck.find(e * a) == ck.end() && to_string(e*a).length() <= to_string(n).length()) {
        tmp.push_back(e * a);
        ck.insert(e * a);
      }
      if(e % 10 == 0) continue;
      if(e < 10) continue;
      string s = to_string(e);
      s = s.back() + s.substr(0, s.length()-1);
      ll k = stoll(s);
      if(ck.find(k) != ck.end()) continue;
      ck.insert(k);
      tmp.push_back(k);
    }
    t.swap(tmp);
    cnt++;
  }
  return 0;
}
