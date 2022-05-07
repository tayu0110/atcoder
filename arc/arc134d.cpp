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
template<class T>
struct heap {
  priority_queue<T, vector<T>, greater<T>> pq;
  heap() : pq() {}
  heap(priority_queue<T, vector<T>, greater<T>> pq) : pq(pq) {}
  void push(T c) { pq.push(c); }
  T top() { return pq.top(); }
  void pop() { pq.pop(); }
  bool empty() { return pq.empty(); }
  int size() { return pq.size(); }
  void swap(heap<T> nt) { pq.swap(nt.pq); }
};
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<ll> a(n), b(n);
  for(int i=0;i<n;i++) cin >> a[i];
  for(int i=0;i<n;i++) cin >> b[i];
  map<ll, int> mp, rmp;
  for(int i=0;i<n;i++) {
    mp[a[i]]++;
    mp[b[i]]++;
  }
  int cnt = 0;
  for(auto &[f, s] : mp) {
    s = cnt;
    rmp[cnt] = f;
    cnt++;
  }
  for(int i=0;i<n;i++) {
    a[i] = mp[a[i]];
    b[i] = mp[b[i]];
  }
  vector<vector<pii>> t(cnt);
  for(int i=0;i<n;i++) {
    t[a[i]].push_back({b[i], i});
  }
  vector<int> ans;
  int back = inf;
  int ni = -1;
  for(int i=0;i<cnt;i++) {
    if(i > back) break;
    else if(i == back) {
      bool flag = true;
      for(auto e : ans) {
        if(b[e] == i) continue;
        if(b[e] < i) break;
        else flag = false;
      }
      if(flag) break;
    }
    if(t[i].empty()) continue;
    pii mn = {inf, inf};
    if(ans.empty()) {
      for(auto e : t[i]) {
        if(e.second > ni) mn = min(mn, e);
      }
      if(mn.first <= i) {
        ans.push_back(mn.second);
        break;
      };
    }
    for(auto [now, idx] : t[i]) {
      if(idx > ni) ans.push_back(idx);
    }
    ni = ans.back();
    back = b[ans[0]];
  }
  for(int i=0;i<ans.size();i++) cout << rmp[a[ans[i]]] << " ";
  for(int i=0;i<ans.size();i++) {
    cout << rmp[b[ans[i]]];
    if(i == ans.size()-1) cout << endl;
    else cout << " ";
  }
  return 0;
}
