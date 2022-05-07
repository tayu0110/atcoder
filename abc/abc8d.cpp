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
map<tuple<int, int, int, int>, ll> memo;
ll solve(int, int, int , int, vector<pii>&);
ll calc(int up, int down, int left, int right, vector<pii> &p) {
  if(!p.size()) return 0;
  if(memo.find({up, down, left, right}) != memo.end()) return memo[{up, down, left, right}];
  else return memo[{up, down, left, right}] = solve(up, down, left, right, p);
}
ll solve(int up, int down, int left, int right, vector<pii> &p) {
  ll res = 0;
  int n = p.size();
  if(!n) return 0;
  for(int i=0;i<n;i++) {
    auto [x, y] = p[i];
    ll tmp = right - left + down - up + 1;
    vector<pii> one, two, three, four;
    for(int j=0;j<n;j++) {
      if(i == j) continue;
      auto [nx, ny] = p[j];
      if(nx < x && ny > y) one.push_back(p[j]);
      else if(nx > x && ny > y) two.push_back(p[j]);
      else if(nx > x && ny < y) three.push_back(p[j]);
      else four.push_back(p[j]);
    }
    tmp += calc(y+1, down, left, x-1, one);
    tmp += calc(y+1, down, x+1, right, two);
    tmp += calc(up, y-1, x+1, right, three);
    tmp += calc(up, y-1, left, x-1, four);
    res = max(res, tmp);
  }
  return res;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int w, h, n;
  cin >> w >> h >> n;
  vector<pii> p(n);
  for(int i=0;i<n;i++) {
    int x, y;
    cin >> x >> y;
    p[i] = {x, y};
  }
  cout << solve(1, h, 1, w, p) << endl;
  return 0;
}
