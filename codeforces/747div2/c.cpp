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
  int t; cin >> t;
  while(t--) {
    int n;
    char c;
    string s;
    cin >> n >> c >> s;
    vector<pii> k(n+1, {0, 0});
    for(int i=0;i<n+1;i++) k[i] = {0, i};
    int cnt = 0;
    set<int> ck;
    for(int i=0;i<n;i++) {
      if(s[i] == c) continue;
      cnt++;
      ck.insert(i+1);
      int t = i+1;
      for(int j=1;j*j<=t;j++) {
        if(t % j) continue;
        k[j].first++;
        if(t/j != j) {
          k[t/j].first++;
        }
      }
    }
    vector<int> res;
    sort(k.begin(), k.end());
    if(k.back().first == 0) {
      cout << 0 << endl;
      continue;
    }
    for(auto [fi, se] : k) {
      if(!se) continue;
      set<int> tmp;
      for(int i=1;i*se<=n;i++) {
        if(ck.find(se*i) != ck.end()) {
          tmp.insert(se*i);
        }
      }
      res.push_back(se);
      if(!tmp.size()) break;
      ck.swap(tmp);
    }
    cout << res.size() << endl;
    for(int i=0;i<res.size();i++) {
      cout << res[i];
      if(i != res.size()-1) cout << " ";
      else cout << endl;
    }
  }
  return 0;
}
