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
vector<int> ans;
vector<bool> ck, tmp;
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
};
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  Graph t(n);
  vector<int> in(n, 0);
  for(int i=0;i<m;i++) {
    int a, b;
    cin >> a >> b;
    a--;b--;
    t[a].push_back(b);
    in[b]++;
  }
  heap<int> hp;
  for(int i=0;i<n;i++) {
    if(!in[i]) hp.push(i);
  }
  vector<int> ans;
  while(!hp.empty()) {
    int now = hp.top();
    hp.pop();
    for(auto e : t[now]) {
      in[e]--;
      if(!in[e]) hp.push(e);
    }
    ans.push_back(now);
  }
  if(ans.size() != n) cout << -1 << endl;
  else {
    for(int i=0;i<n;i++) {
      cout << ans[i]+1;
      if(i != n-1) cout << " ";
      else cout << endl;
    }
  }
  return 0;
}
