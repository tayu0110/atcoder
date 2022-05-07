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
  int n;
  string s;
  cin >> n >> s;
  vector<set<int>> t(26);
  for(int i=0;i<n;i++) {
    t[s[i]-'a'].insert(i);
  }
  int back = n;
  for(int i=0;i<n;i++) {
    for(char c='a';c<s[i];c++) {
      int idx = c - 'a';
      if(t[idx].empty()) continue;
      auto it = t[idx].lower_bound(back);
      if(it == t[idx].begin()) continue;
      it--;
      int to = *it;
      if(to < i || to > back) continue;
      swap(s[i], s[to]);
      t[idx].erase(to);
      back = to;
      break;
    }
  }
  cout << s << endl;
  return 0;
}
