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
vector<int> nt;
vector<bool> reach;
bool dfs(int now, set<int> &ck, Graph &t) {
  reach[now] = true;
  bool res = false;
  if(ck.find(now) != ck.end()) {
    ck.erase(now);
    res = true;
  }
  for(auto to : t[now]) {
    if(reach[to]) continue;
    if(dfs(to, ck, t)) {
      res = true;
    }
  }
  if(res) nt.push_back(now);
  return res;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  Graph t(n);
  for(int i=0;i<m;i++) {
    int u, v;
    cin >> u >> v;
    u--; v--;
    t[u].push_back(v);
    t[v].push_back(u);
  }
  string s;
  cin >> s;
  bool f = false;
  for(int i=0;i<n;i++) {
    f |= s[i] == '1';
  }
  if(!f) {
    cout << 0 << endl;
    cout << endl;
    return 0;
  }
  set<int> ck;
  for(int i=0;i<n;i++) if(s[i] == '1') ck.insert(i);
  int start = *ck.begin();
  if(ck.size() == 1) {
    int to = t[start][0];
    cout << 3 << endl;
    cout << to << " " << start << " " << to << endl;
    return 0;
  }
  reach.assign(n, false);
  set<int> tmp = ck;
  dfs(start, tmp, t);
  reverse(nt.begin(), nt.end());
  vector<int> res;
  vector<int> cnt(n);
  for(int i=0;i<nt.size();i++) {
    if(ck.find(nt[i]) != ck.end()) {
      res.push_back(nt[i]);
      if(ck.find(nt[i-1]) == ck.end() && cnt[nt[i-1]] % 2 != 0) {
        res.push_back(nt[i-1]);
        res.push_back(nt[i]);
        cnt[nt[i-1]]++;
        cnt[nt[i]]++;
        ck.erase(nt[i]);
      }
    } else {
      res.push_back(nt[i]);
      cnt[nt[i]]++;
      if(ck.find(nt[i-1]) == ck.end() && cnt[nt[i-1]] % 2 != 0) {
        res.push_back(nt[i-1]);
        res.push_back(nt[i]);
        cnt[nt[i-1]]++;
        cnt[nt[i]]++;
      }
    }
  }
  cout << res.size() << endl;
  for(int i=0;i<res.size();i++) {
    if(i) cout << " "; cout << res[i]+1;
  }
  cout << endl;
  return 0;
}
