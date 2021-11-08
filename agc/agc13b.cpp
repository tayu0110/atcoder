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
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  Graph t(n);
  for(int i=0;i<m;i++) {
    int a, b;
    cin >> a >> b;
    a--;b--;
    t[a].push_back(b);
    t[b].push_back(a);
  }
  deque<int> ans;
  vector<bool> ck(n, false);
  ck[0] = true;
  ans.push_back(0);
  for(int i=0;i<n;i++) {
    if(ans.size() == 1) {
      int f = ans.front();
      for(auto e : t[f]) {
        if(ck[e]) continue;
        ans.push_back(e);
        ck[e] = true;
        break;
      }
    } else {
      int f = ans.front();
      for(auto e : t[f]) {
        if(ck[e]) continue;
        ans.push_front(e);
        ck[e] = true;
        break;
      }
      int b = ans.back();
      for(auto e : t[b]) {
        if(ck[e]) continue;
        ans.push_back(e);
        ck[e] = true;
        break;
      }
    }
  }
  cout << ans.size() << endl;
  while(!ans.empty()) {
    cout << ans.front()+1;
    ans.pop_front();
    if(ans.empty()) cout << endl;
    else cout << " ";
  }
  return 0;
}
