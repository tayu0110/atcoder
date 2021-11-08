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
  ll k;
  cin >> n >> k;
  vector<int> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  map<int, queue<int>> mp;
  for(int i=0;i<n;i++) mp[a[i]].push(i);
  vector<int> t(n);
  for(int i=0;i<n;i++) {
    int now = a[i];
    int ft = mp[now].front();
    mp[now].pop();
    if(ft == i && !mp[now].empty()) {
        mp[now].push(ft);
        ft = mp[now].front();
    }
    t[i] = ft;
  }
  int cnt = 1;
  int now = 0;
  while(true) {
    int nt = t[now];
    if(nt <= now) cnt++;
    if(nt == n-1) break;
    now = (nt+1) % n;
  }
  k--;
  k %= cnt;
  now = 0;
  while(k) {
    int nt = t[now];
    if(nt <= now) k--;
    now = (nt+1) % n;
  }
  vector<int> ans;
  while(now < n) {
    int nt = t[now];
    if(nt <= now) {
      ans.push_back(now);
      now++;
    } else now = (nt+1)%n;
    if(nt == n-1) break;
  }
  for(int i=0;i<ans.size();i++) {
    cout << a[ans[i]];
    if(i != ans.size()-1) cout << " ";
    else cout << endl;
  }
  return 0;
}
