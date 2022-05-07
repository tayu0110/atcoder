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
  int n;
  cin >> n;
  string ans;
  vector<pii> t(7);
  int rem = 0;
  while(true) {
    int len = ans.length();
    if((len+1) * (len+2) / 2 > n) break;
    ans += '7';
    t[0] = {t[0].first+1, 0};
  }
  n -= ans.length() * (ans.length()+1) / 2;
  for(int i=1;i<t.size();i++) t[i] = {0, i};
  while(n) {
    vector<pii> tmp(7);
    for(int i=0;i<7;i++) {
      int ni = (t[i].second * 10) % 7;
      tmp[ni] = {tmp[ni].first + t[i].first, ni};
    }
    t.swap(tmp);
    sort(t.begin(), t.end(), greater<pii>());
    for(auto &[f, s] : t) {
      if(s == 0) continue;
      if(f > n) continue;
      int k = 10 * rem;
      for(int i=1;i<10;i++) {
        if((k + i) % 7 == s) {
          ans += (char)(i + '0');
          break;
        }
      }
      n -= f;
      f++;
      rem = s;
      break;
    }
    DEBUG(rem);
    DEBUG_EN(ans);
  }
  cout << ans << endl;
  return 0;
}
