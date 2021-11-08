#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<deque>
#include<set>
#include<stack>
#include<numeric>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>
#include<cassert>

using namespace std;

#define DEBUG(var) cout << #var << ": " << var << " ";
#define DEBUG_EN(var) cout << #var << ": " << var << endl;

struct Edge {
  int to;
  long long weight;
  Edge() : to(0), weight(0) {}
  Edge(int to, long long weight) : to(to), weight(weight) {}
  Edge(const Edge& e) {
    to = e.to;
    weight = e.weight;
  }
  bool operator>(const Edge &e) const { return weight > e.weight; }
  bool operator<(const Edge &e) const { return weight < e.weight; }
  bool operator==(const Edge &e) const { return weight == e.weight; }
  bool operator<=(const Edge &e) const { return weight <= e.weight; }
  bool operator>=(const Edge &e) const { return weight >= e.weight; }
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;
using heap = priority_queue<int, vector<int>, greater<int>>;

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;
vector<int> memo;
vector<string> s;
map<string, vector<int>> mp;
int dfs(int now, int dep, set<int>& ck, bool &en_loop) {
  if(ck.find(now) != ck.end()) {
    if(dep % 2 == 1) en_loop = true;
    return -1;
  }
  ck.insert(now);
  bool f = false;
  int len = s[now].length();
  int res = 0;
  string tail = s[now].substr(len-3, 3);
  for(auto e : mp[tail]) {
    f = true;
    int ret;
    if(memo[e] > -inf) ret = memo[e];
    else ret = dfs(e, dep+1, ck, en_loop);
    if(ret >= 0) res |= (ret+1)%2;
  }
  if(!f) res = 1;
  if(res == 1) en_loop = false;
  if(en_loop && res == 0) res = -1;
  return memo[now] = res;
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  s.resize(n);
  for(int i=0;i<n;i++) {
    cin >> s[i];
    string pre = s[i].substr(0, 3);
    mp[pre].push_back(i);
  }
  memo.assign(n, -inf);
  for(int i=0;i<n;i++) {
    if(memo[i] > -inf) continue;
    set<int> ck;
    bool en_loop = false;
    dfs(i, 0, ck, en_loop);
  }
  for(auto e : memo) {
    if(e > 0) cout << "Takahashi" << endl;
    else if(e == 0) cout << "Aoki" << endl;
    else cout << "Draw" << endl;
  }
  return 0;
}
