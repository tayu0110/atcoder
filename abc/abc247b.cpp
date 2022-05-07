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
  vector<pair<string, string>> p(n);
  for(int i=0;i<n;i++) {
    string s, t;
    cin >> s >> t;
    p[i] = {s, t};
  }
  for(int i=0;i<n;i++) {
    auto [s, t] = p[i];
    bool f1 = true, f2 = true;
    for(int j=0;j<n;j++) {
      if(i == j) continue;
      auto [ns, nt] = p[j];
      if(ns == s || nt == s) f1 = false;
      if(nt == t || ns == t) f2 = false;
    }
    if(!f1 && !f2) {
      cout << "No" << endl;
      return 0;
    }
  }
  cout << "Yes" << endl;
  return 0;
}
